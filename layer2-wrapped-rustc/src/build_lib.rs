use syn::{parse_file, Item, File};
use quote::quote;

#[derive(Debug, Clone)]
pub struct TransformationStep {
    pub name: String,
    pub content: String,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug)]
pub struct ProcessingAudit {
    pub original_content: String,
    pub steps: Vec<TransformationStep>,
    pub final_success: bool,
    pub bisection_log: Vec<String>,
}

impl ProcessingAudit {
    pub fn new(content: &str) -> Self {
        Self {
            original_content: content.to_string(),
            steps: Vec::new(),
            final_success: false,
            bisection_log: Vec::new(),
        }
    }

    pub fn add_step(&mut self, name: &str, content: String, success: bool, error: Option<String>) {
        self.steps.push(TransformationStep {
            name: name.to_string(),
            content,
            success,
            error,
        });
    }

    pub fn test_parse(&self, content: &str) -> (bool, Option<String>) {
        match parse_file(content) {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        }
    }

    pub fn bisect_transformations(&mut self) -> Result<String, String> {
        self.bisection_log.push("Starting bisection process".to_string());
        
        // Test original
        let (original_ok, _) = self.test_parse(&self.original_content);
        if !original_ok {
            return Err("Original content doesn't parse".to_string());
        }

        // Try all transformations together first
        let mut current = self.original_content.clone();
        let mut successful_steps = Vec::new();

        // Apply each transformation one by one and test
        for (i, step) in self.steps.iter().enumerate() {
            let test_content = apply_transformation_by_name(&current, &step.name);
            let (parse_ok, parse_error) = self.test_parse(&test_content);
            
            if parse_ok {
                self.bisection_log.push(format!("✅ Step {}: {} - SUCCESS", i+1, step.name));
                current = test_content;
                successful_steps.push(step.name.clone());
            } else {
                self.bisection_log.push(format!("❌ Step {}: {} - FAILED: {}", 
                    i+1, step.name, parse_error.unwrap_or("Unknown error".to_string())));
                // Skip this transformation
            }
        }

        self.bisection_log.push(format!("Bisection complete. Applied {} out of {} transformations", 
            successful_steps.len(), self.steps.len()));

        Ok(current)
    }
}

pub fn add_prelude(content: &str) -> String {
    match add_prelude_syn(content) {
        Ok(result) => result,
        Err(_) => {
            // Fallback to original method if syn parsing fails
            format!("use split_decls_genesis::ourprelude::*;\n{}", content)
        }
    }
}

fn add_prelude_syn(content: &str) -> Result<String, syn::Error> {
    let mut file: File = parse_file(content)?;
    
    // Create the prelude use statement
    let prelude_use: Item = syn::parse_quote! {
        use split_decls_genesis::ourprelude::*;
    };
    
    // Insert at the beginning of items (after attributes and comments)
    file.items.insert(0, prelude_use);
    
    Ok(quote!(#file).to_string())
}

pub fn fix_file_paths(content: &str) -> String {
    content.replace("\"../messages.ftl\"", "\"messages.ftl\"")
}

pub fn fix_env_vars(content: &str) -> String {
    content.replace("env ! (\"CFG_RELEASE_CHANNEL\")", "\"dev\"")
}

pub fn fix_attribute_spacing(content: &str) -> String {
    content.replace("# [", "#[")
}

pub fn remove_crate_attrs(content: &str) -> String {
    content.replace("# [allow (internal_features)] # [allow (rustc :: untranslatable_diagnostic)] # [doc (html_root_url = \"https://doc.rust-lang.org/nightly/nightly-rustc/\")] # [doc (rust_logo)] # [feature (decl_macro)] # [feature (panic_backtrace_config)] # [feature (panic_update_hook)] # [feature (rustdoc_internals)] # [feature (try_blocks)] ", "")
}

fn apply_transformation_by_name(content: &str, name: &str) -> String {
    match name {
        "add_prelude" => add_prelude(content),
        "fix_file_paths" => fix_file_paths(content),
        "fix_env_vars" => fix_env_vars(content),
        "fix_attribute_spacing" => fix_attribute_spacing(content),
        "remove_crate_attrs" => remove_crate_attrs(content),
        _ => content.to_string(),
    }
}

pub fn wrap_item(item: &Item) -> String {
    match item {
        Item::Fn(item_fn) => {
            let fn_name = &item_fn.sig.ident;
            format!("mkfn!{{
    {}_introspect!();
    {}
}}", fn_name, quote::ToTokens::to_token_stream(item))
        }
        Item::Struct(_) => {
            format!("mkitem!{{mkstruct!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Enum(_) => {
            format!("mkitem!{{mkenum!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Trait(_) => {
            format!("mkitem!{{mktrait!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        Item::Impl(_) => {
            format!("mkitem!{{mkimpl!{{{}}}}}", quote::ToTokens::to_token_stream(item))
        }
        _ => format!("mkitem!{{{}}}", quote::ToTokens::to_token_stream(item))
    }
}

pub fn process_content_with_audit(content: &str) -> Result<(String, ProcessingAudit), Box<dyn std::error::Error>> {
    let mut audit = ProcessingAudit::new(content);
    
    // Define transformation steps
    let transformations = vec![
        "add_prelude",
        "fix_file_paths", 
        "fix_env_vars",
        "fix_attribute_spacing",
        "remove_crate_attrs",
    ];

    // Test original content
    let (original_ok, original_error) = audit.test_parse(content);
    if !original_ok {
        return Err(format!("Original content doesn't parse: {}", 
            original_error.unwrap_or("Unknown error".to_string())).into());
    }

    // Apply transformations and record each step
    let mut current = content.to_string();
    for transform_name in &transformations {
        let transformed = apply_transformation_by_name(&current, transform_name);
        let (parse_ok, parse_error) = audit.test_parse(&transformed);
        
        audit.add_step(transform_name, transformed.clone(), parse_ok, parse_error);
        
        if parse_ok {
            current = transformed;
        }
    }

    // Try to parse the final result
    let (final_ok, _) = audit.test_parse(&current);
    
    if !final_ok {
        // If final result doesn't parse, run bisection
        match audit.bisect_transformations() {
            Ok(bisected_result) => {
                audit.final_success = true;
                Ok((bisected_result, audit))
            }
            Err(e) => Err(e.into())
        }
    } else {
        audit.final_success = true;
        Ok((current, audit))
    }
}

pub fn process_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (result, _audit) = process_content_with_audit(content)?;
    Ok(result)
}
