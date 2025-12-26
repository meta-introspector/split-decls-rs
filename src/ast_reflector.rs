use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use syn::{visit_mut::VisitMut, *};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstProbe {
    pub name: String,
    pub node_type: AstNodeType,
    pub filter: ProbeFilter,
    pub action: ProbeAction,
    pub enabled: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AstNodeType {
    Function,
    Struct,
    Enum,
    Impl,
    Trait,
    Module,
    Use,
    Const,
    Static,
    Type,
    Macro,
    Expr,
    Stmt,
    Pat,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeFilter {
    pub name_pattern: Option<String>,
    pub visibility: Option<String>, // pub, crate, etc.
    pub attributes: Vec<String>,
    pub contains_text: Option<String>,
    pub complexity_threshold: Option<f64>,
    pub layer: Option<String>, // AST, HIR, MIR
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProbeAction {
    Log { message: String },
    AddAttribute { attr: String },
    WrapFunction { wrapper: String },
    InjectCode { code: String, position: InjectionPosition },
    Transform { macro_name: String, args: Vec<String> },
    Collect { field: String },
    Enhance { enhancement_type: String, data: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjectionPosition {
    Before,
    After,
    Inside,
    Replace,
}

#[derive(Debug, Default)]
pub struct AstReflector {
    probes: Vec<AstProbe>,
    collected_data: HashMap<String, Vec<String>>,
    transformation_count: HashMap<String, usize>,
    current_file: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ProbeContext {
    pub file_path: PathBuf,
    pub node_name: String,
    pub node_type: AstNodeType,
    pub attributes: Vec<String>,
    pub visibility: String,
    pub complexity: f64,
}

impl AstReflector {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn load_probes(&mut self, config_path: &Path) -> Result<()> {
        let content = fs::read_to_string(config_path)?;
        self.probes = serde_json::from_str(&content)?;
        println!("🔍 Loaded {} AST probes", self.probes.len());
        Ok(())
    }
    
    pub fn add_probe(&mut self, probe: AstProbe) {
        self.probes.push(probe);
        self.probes.sort_by_key(|p| p.priority);
    }
    
    pub fn reflect_directory(&mut self, dir: &Path) -> Result<()> {
        println!("🌲 Reflecting AST in directory: {:?}", dir);
        
        for entry in walkdir::WalkDir::new(dir) {
            let entry = entry?;
            if entry.file_type().is_file() && 
               entry.path().extension().map_or(false, |ext| ext == "rs") {
                self.reflect_file(entry.path())?;
            }
        }
        
        self.print_summary();
        Ok(())
    }
    
    pub fn reflect_file(&mut self, file_path: &Path) -> Result<()> {
        self.current_file = file_path.to_path_buf();
        
        let content = fs::read_to_string(file_path)?;
        let mut syntax_tree: File = syn::parse_str(&content)?;
        
        // Apply probes to the AST
        self.visit_file_mut(&mut syntax_tree);
        
        // Write back transformed file if any changes were made
        if self.has_transformations() {
            let transformed = syntax_tree.to_token_stream().to_string();
            let output_path = self.get_output_path(file_path);
            fs::create_dir_all(output_path.parent().unwrap())?;
            fs::write(&output_path, transformed)?;
            println!("✨ Transformed: {:?} -> {:?}", file_path, output_path);
        }
        
        Ok(())
    }
    
    fn has_transformations(&self) -> bool {
        self.transformation_count.values().sum::<usize>() > 0
    }
    
    fn get_output_path(&self, input_path: &Path) -> PathBuf {
        let mut output = PathBuf::from("ast_reflected");
        output.push(input_path.strip_prefix(".").unwrap_or(input_path));
        output
    }
    
    fn create_probe_context(&self, node_name: &str, node_type: AstNodeType, attrs: &[Attribute]) -> ProbeContext {
        ProbeContext {
            file_path: self.current_file.clone(),
            node_name: node_name.to_string(),
            node_type,
            attributes: attrs.iter().map(|a| a.to_token_stream().to_string()).collect(),
            visibility: "private".to_string(), // TODO: extract actual visibility
            complexity: self.calculate_complexity(node_name),
        }
    }
    
    fn calculate_complexity(&self, _node_name: &str) -> f64 {
        // Simple complexity metric - could be enhanced
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        _node_name.hash(&mut hasher);
        (hasher.finish() % 100) as f64 / 10.0
    }
    
    fn apply_probes(&mut self, context: &ProbeContext) -> Vec<TokenStream> {
        let mut injections = Vec::new();
        
        for probe in &self.probes.clone() {
            if !probe.enabled {
                continue;
            }
            
            if self.matches_filter(&probe.filter, context) {
                if let Some(injection) = self.execute_probe_action(&probe.action, context) {
                    injections.push(injection);
                    *self.transformation_count.entry(probe.name.clone()).or_insert(0) += 1;
                }
            }
        }
        
        injections
    }
    
    fn matches_filter(&self, filter: &ProbeFilter, context: &ProbeContext) -> bool {
        // Name pattern matching
        if let Some(pattern) = &filter.name_pattern {
            if !context.node_name.contains(pattern) {
                return false;
            }
        }
        
        // Visibility matching
        if let Some(vis) = &filter.visibility {
            if context.visibility != *vis {
                return false;
            }
        }
        
        // Attribute matching
        for required_attr in &filter.attributes {
            if !context.attributes.iter().any(|a| a.contains(required_attr)) {
                return false;
            }
        }
        
        // Complexity threshold
        if let Some(threshold) = filter.complexity_threshold {
            if context.complexity < threshold {
                return false;
            }
        }
        
        true
    }
    
    fn execute_probe_action(&mut self, action: &ProbeAction, context: &ProbeContext) -> Option<TokenStream> {
        match action {
            ProbeAction::Log { message } => {
                println!("🔍 [{}] {}: {}", context.node_type.as_str(), context.node_name, message);
                None
            }
            
            ProbeAction::AddAttribute { attr } => {
                let attr_tokens: TokenStream = attr.parse().ok()?;
                Some(quote! { #attr_tokens })
            }
            
            ProbeAction::WrapFunction { wrapper } => {
                let wrapper_tokens: TokenStream = format!("#[{}]", wrapper).parse().ok()?;
                Some(wrapper_tokens)
            }
            
            ProbeAction::InjectCode { code, position: _ } => {
                let code_tokens: TokenStream = code.parse().ok()?;
                Some(code_tokens)
            }
            
            ProbeAction::Transform { macro_name, args } => {
                let transform_code = format!("{}!({})", macro_name, args.join(", "));
                let code_tokens: TokenStream = transform_code.parse().ok()?;
                Some(code_tokens)
            }
            
            ProbeAction::Collect { field } => {
                self.collected_data.entry(field.clone())
                    .or_insert_with(Vec::new)
                    .push(context.node_name.clone());
                None
            }
            
            ProbeAction::Enhance { enhancement_type, data } => {
                let enhancement_comment = format!("// Enhanced: {} - {}", enhancement_type, data);
                let tokens: TokenStream = enhancement_comment.parse().ok()?;
                Some(tokens)
            }
        }
    }
    
    fn print_summary(&self) {
        println!("\n📊 AST Reflection Summary:");
        println!("========================");
        
        for (probe_name, count) in &self.transformation_count {
            println!("  🔍 {}: {} transformations", probe_name, count);
        }
        
        if !self.collected_data.is_empty() {
            println!("\n📋 Collected Data:");
            for (field, values) in &self.collected_data {
                println!("  📊 {}: {} items", field, values.len());
            }
        }
    }
}

impl AstNodeType {
    fn as_str(&self) -> &'static str {
        match self {
            AstNodeType::Function => "Function",
            AstNodeType::Struct => "Struct",
            AstNodeType::Enum => "Enum",
            AstNodeType::Impl => "Impl",
            AstNodeType::Trait => "Trait",
            AstNodeType::Module => "Module",
            AstNodeType::Use => "Use",
            AstNodeType::Const => "Const",
            AstNodeType::Static => "Static",
            AstNodeType::Type => "Type",
            AstNodeType::Macro => "Macro",
            AstNodeType::Expr => "Expr",
            AstNodeType::Stmt => "Stmt",
            AstNodeType::Pat => "Pat",
            AstNodeType::All => "All",
        }
    }
}

impl VisitMut for AstReflector {
    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) {
        let context = self.create_probe_context(&node.sig.ident.to_string(), AstNodeType::Function, &node.attrs);
        let injections = self.apply_probes(&context);
        
        // Apply injections as comments for now (safer approach)
        for injection in injections {
            let comment = format!("AST Probe Function: {}", injection.to_string());
            let doc_attr = quote! { #[doc = #comment] };
            if let Ok(attr) = syn::parse2(doc_attr) {
                node.attrs.push(attr);
            }
        }
        
        syn::visit_mut::visit_item_fn_mut(self, node);
    }
    
    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        let context = self.create_probe_context(&node.ident.to_string(), AstNodeType::Struct, &node.attrs);
        let injections = self.apply_probes(&context);
        
        for injection in injections {
            let comment = format!("AST Probe Struct: {}", injection.to_string());
            let doc_attr = quote! { #[doc = #comment] };
            if let Ok(attr) = syn::parse2(doc_attr) {
                node.attrs.push(attr);
            }
        }
        
        syn::visit_mut::visit_item_struct_mut(self, node);
    }
    
    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        let context = self.create_probe_context(&node.ident.to_string(), AstNodeType::Enum, &node.attrs);
        let injections = self.apply_probes(&context);
        
        for injection in injections {
            let comment = format!("AST Probe Enum: {}", injection.to_string());
            let doc_attr = quote! { #[doc = #comment] };
            if let Ok(attr) = syn::parse2(doc_attr) {
                node.attrs.push(attr);
            }
        }
        
        syn::visit_mut::visit_item_enum_mut(self, node);
    }
    
    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        let type_name = node.self_ty.to_token_stream().to_string();
        let context = self.create_probe_context(&type_name, AstNodeType::Impl, &node.attrs);
        let injections = self.apply_probes(&context);
        
        for injection in injections {
            let comment = format!("AST Probe Impl: {}", injection.to_string());
            let doc_attr = quote! { #[doc = #comment] };
            if let Ok(attr) = syn::parse2(doc_attr) {
                node.attrs.push(attr);
            }
        }
        
        syn::visit_mut::visit_item_impl_mut(self, node);
    }
    
    fn visit_item_trait_mut(&mut self, node: &mut ItemTrait) {
        let context = self.create_probe_context(&node.ident.to_string(), AstNodeType::Trait, &node.attrs);
        let injections = self.apply_probes(&context);
        
        for injection in injections {
            let comment = format!("AST Probe Trait: {}", injection.to_string());
            let doc_attr = quote! { #[doc = #comment] };
            if let Ok(attr) = syn::parse2(doc_attr) {
                node.attrs.push(attr);
            }
        }
        
        syn::visit_mut::visit_item_trait_mut(self, node);
    }
}

// Example probe configurations
pub fn create_example_probes() -> Vec<AstProbe> {
    vec![
        AstProbe {
            name: "debug_functions".to_string(),
            node_type: AstNodeType::Function,
            filter: ProbeFilter {
                name_pattern: Some("debug".to_string()),
                visibility: None,
                attributes: vec![],
                contains_text: None,
                complexity_threshold: None,
                layer: None,
            },
            action: ProbeAction::AddAttribute {
                attr: "#[instrument]".to_string(),
            },
            enabled: true,
            priority: 1,
        },
        
        AstProbe {
            name: "collect_structs".to_string(),
            node_type: AstNodeType::Struct,
            filter: ProbeFilter {
                name_pattern: None,
                visibility: Some("pub".to_string()),
                attributes: vec![],
                contains_text: None,
                complexity_threshold: None,
                layer: None,
            },
            action: ProbeAction::Collect {
                field: "public_structs".to_string(),
            },
            enabled: true,
            priority: 2,
        },
        
        AstProbe {
            name: "enhance_complex_functions".to_string(),
            node_type: AstNodeType::Function,
            filter: ProbeFilter {
                name_pattern: None,
                visibility: None,
                attributes: vec![],
                contains_text: None,
                complexity_threshold: Some(5.0),
                layer: None,
            },
            action: ProbeAction::Enhance {
                enhancement_type: "complexity_warning".to_string(),
                data: "High complexity function detected".to_string(),
            },
            enabled: true,
            priority: 3,
        },
        
        AstProbe {
            name: "wrap_error_functions".to_string(),
            node_type: AstNodeType::Function,
            filter: ProbeFilter {
                name_pattern: Some("error".to_string()),
                visibility: None,
                attributes: vec![],
                contains_text: None,
                complexity_threshold: None,
                layer: None,
            },
            action: ProbeAction::WrapFunction {
                wrapper: "error_handler".to_string(),
            },
            enabled: true,
            priority: 4,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_probe_creation() {
        let probes = create_example_probes();
        assert_eq!(probes.len(), 4);
        assert!(probes[0].enabled);
    }
    
    #[test]
    fn test_filter_matching() {
        let reflector = AstReflector::new();
        let context = ProbeContext {
            file_path: PathBuf::from("test.rs"),
            node_name: "debug_function".to_string(),
            node_type: AstNodeType::Function,
            attributes: vec![],
            visibility: "pub".to_string(),
            complexity: 3.0,
        };
        
        let filter = ProbeFilter {
            name_pattern: Some("debug".to_string()),
            visibility: None,
            attributes: vec![],
            contains_text: None,
            complexity_threshold: None,
            layer: None,
        };
        
        assert!(reflector.matches_filter(&filter, &context));
    }
}
