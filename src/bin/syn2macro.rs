use anyhow::Result;
use clap::{Arg, Command};
use std::fs;
use std::path::Path;
use proc_macro2::TokenStream;
use quote::quote;

use split_decls_rs::syn2macro::{Syn2MacroConverter, DefaultSecurity, StrictSecurity, AstOperation};

fn main() -> Result<()> {
    let matches = Command::new("syn2macro")
        .version("0.1.0")
        .about("Convert syn-based code to universal trait-based macros")
        .arg(Arg::new("input")
            .short('i')
            .long("input")
            .value_name("FILE")
            .help("Input Rust file to convert")
            .required(true))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file for converted traits"))
        .arg(Arg::new("security")
            .short('s')
            .long("security")
            .value_name("LEVEL")
            .help("Security level: default, strict")
            .default_value("default"))
        .arg(Arg::new("context")
            .short('c')
            .long("context")
            .value_name("CONTEXT")
            .help("Execution context: compiler, syn, abstract")
            .default_value("syn"))
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");
    let security_level = matches.get_one::<String>("security").unwrap();
    let context = matches.get_one::<String>("context").unwrap();

    println!("🔄 syn2macro: Converting {} to universal traits", input_file);
    println!("📋 Security level: {}", security_level);
    println!("🎯 Target context: {}", context);

    // Read input file
    let input_content = fs::read_to_string(input_file)?;
    let input_tokens: TokenStream = input_content.parse()?;

    // Create security context
    let security = match security_level.as_str() {
        "strict" => Box::new(StrictSecurity::new(vec![
            AstOperation::ParseItem,
            AstOperation::TransformItem,
            AstOperation::GenerateCode,
        ])),
        _ => Box::new(DefaultSecurity),
    };

    // Create converter
    let converter = Syn2MacroConverter::new(security);

    // Convert to universal traits
    let trait_code = converter.convert_syn_to_traits(input_tokens)?;

    // Add context-specific wrapper
    let final_code = match context.as_str() {
        "compiler" => wrap_for_compiler_context(trait_code),
        "abstract" => wrap_for_abstract_engine(trait_code),
        _ => wrap_for_syn_context(trait_code),
    };

    // Output result
    let output_content = final_code.to_string();
    
    if let Some(output_file) = output_file {
        fs::write(output_file, &output_content)?;
        println!("✅ Converted traits written to {}", output_file);
    } else {
        println!("📄 Generated universal traits:");
        println!("{}", output_content);
    }

    // Generate usage example
    generate_usage_example(&input_file, context)?;

    Ok(())
}

fn wrap_for_compiler_context(trait_code: TokenStream) -> TokenStream {
    quote! {
        use rustc_ast::TokenStream as RustcTokenStream;
        use rustc_span::Span;
        
        pub struct CompilerContext;
        
        impl UniversalAst for CompilerContext {
            type TokenStream = RustcTokenStream;
            type Item = rustc_ast::Item;
            type Error = rustc_errors::DiagnosticBuilder;
            
            fn parse_item(&self, input: Self::TokenStream) -> Result<Self::Item, Self::Error> {
                // Compiler-specific parsing
                todo!("Implement rustc parsing")
            }
            
            fn transform_item(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
                // Direct compiler transformation
                Ok(item)
            }
            
            fn generate_code(&self, item: Self::Item) -> Self::TokenStream {
                // Generate rustc TokenStream
                todo!("Implement rustc code generation")
            }
        }
        
        #trait_code
    }
}

fn wrap_for_syn_context(trait_code: TokenStream) -> TokenStream {
    quote! {
        use syn::{Item, parse2};
        use proc_macro2::TokenStream;
        use quote::ToTokens;
        
        pub struct SynContext;
        
        impl UniversalAst for SynContext {
            type TokenStream = TokenStream;
            type Item = Item;
            type Error = syn::Error;
            
            fn parse_item(&self, input: Self::TokenStream) -> Result<Self::Item, Self::Error> {
                parse2(input)
            }
            
            fn transform_item(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
                Ok(item)
            }
            
            fn generate_code(&self, item: Self::Item) -> Self::TokenStream {
                item.to_token_stream()
            }
        }
        
        #trait_code
    }
}

fn wrap_for_abstract_engine(trait_code: TokenStream) -> TokenStream {
    quote! {
        use serde::{Serialize, Deserialize};
        
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct AbstractTokenStream {
            pub tokens: Vec<AbstractToken>,
        }
        
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub enum AbstractToken {
            Ident(String),
            Literal(String),
            Punct(char),
            Group { delimiter: char, tokens: Vec<AbstractToken> },
        }
        
        pub struct AbstractEngine {
            pub security_level: u8,
            pub allowed_operations: Vec<String>,
        }
        
        impl UniversalAst for AbstractEngine {
            type TokenStream = AbstractTokenStream;
            type Item = AbstractItem;
            type Error = AbstractError;
            
            fn parse_item(&self, input: Self::TokenStream) -> Result<Self::Item, Self::Error> {
                // Abstract parsing with security checks
                todo!("Implement abstract parsing")
            }
            
            fn transform_item(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
                // Sandboxed transformation
                Ok(item)
            }
            
            fn generate_code(&self, item: Self::Item) -> Self::TokenStream {
                // Generate abstract representation
                todo!("Implement abstract code generation")
            }
        }
        
        #[derive(Serialize, Deserialize, Debug, Clone)]
        pub struct AbstractItem {
            pub kind: String,
            pub data: serde_json::Value,
        }
        
        #[derive(Debug)]
        pub struct AbstractError(pub String);
        
        #trait_code
    }
}

fn generate_usage_example(input_file: &str, context: &str) -> Result<()> {
    let example_file = format!("{}_example.rs", input_file.trim_end_matches(".rs"));
    
    let example_code = match context {
        "compiler" => quote! {
            // Example: Using in compiler plugin
            fn main() {
                let ctx = CompilerContext;
                let input = get_rustc_tokenstream();
                let result = ctx.parse_item(input).unwrap();
                println!("Processed in compiler context: {:?}", result);
            }
        },
        "abstract" => quote! {
            // Example: Using in abstract engine
            fn main() {
                let engine = AbstractEngine {
                    security_level: 2,
                    allowed_operations: vec!["parse".to_string(), "transform".to_string()],
                };
                let input = AbstractTokenStream { tokens: vec![] };
                let result = engine.parse_item(input).unwrap();
                println!("Processed in abstract engine: {:?}", result);
            }
        },
        _ => quote! {
            // Example: Using in syn context
            fn main() {
                let ctx = SynContext;
                let input: TokenStream = "fn hello() {}".parse().unwrap();
                let result = ctx.parse_item(input).unwrap();
                println!("Processed in syn context: {:?}", result);
            }
        },
    };
    
    fs::write(&example_file, example_code.to_string())?;
    println!("📝 Usage example written to {}", example_file);
    
    Ok(())
}
