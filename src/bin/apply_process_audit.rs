use std::path::Path;
use syn::{visit_mut::VisitMut, *};
use quote::quote;
use anyhow::Result;

struct ProcessAuditTransformer;

impl VisitMut for ProcessAuditTransformer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Call(call) => {
                if let Expr::Path(path_expr) = &*call.func {
                    let path_str = quote!(#path_expr).to_string();
                    if path_str.contains("std::process::Command") || 
                       path_str.contains("Command::new") ||
                       path_str.contains("execute") {
                        
                        let original = call.clone();
                        let new_call = quote! {
                            audit_execute!(#original)
                        };
                        
                        if let Ok(new_expr) = syn::parse2::<Expr>(new_call) {
                            *expr = new_expr;
                            return;
                        }
                    }
                }
            }
            Expr::MethodCall(method_call) => {
                let method_name = method_call.method.to_string();
                if method_name == "execute" || method_name == "spawn" || method_name == "output" {
                    let original = method_call.clone();
                    let new_call = quote! {
                        audit_execute!(#original)
                    };
                    
                    if let Ok(new_expr) = syn::parse2::<Expr>(new_call) {
                        *expr = new_expr;
                        return;
                    }
                }
            }
            _ => {}
        }
        
        syn::visit_mut::visit_expr_mut(self, expr);
    }
}

fn transform_file(file_path: &Path) -> Result<String> {
    let content = #[syscall="read"]
    std::fs::read_to_string(file_path)?;
    let mut syntax_tree: File = syn::parse_str(&content)?;
    
    let mut transformer = ProcessAuditTransformer;
    transformer.visit_file_mut(&mut syntax_tree);
    
    let audit_macro = quote! {
        macro_rules! audit_execute {
            ($cmd:expr) => {{
                let start_time = std::time::Instant::now();
                println!("🔍 AUDIT: Executing command at {:?}", start_time);
                println!("📋 Command: {:?}", stringify!($cmd));
                
                let result = $cmd;
                
                let duration = start_time.elapsed();
                match &result {
                    Ok(output) => {
                        println!("✅ SUCCESS: Command completed in {:?}", duration);
                        println!("📤 Exit status: {:?}", output.status);
                        if !output.stdout.is_empty() {
                            println!("📝 Stdout: {}", String::from_utf8_lossy(&output.stdout));
                        }
                        if !output.stderr.is_empty() {
                            println!("⚠️ Stderr: {}", String::from_utf8_lossy(&output.stderr));
                        }
                    }
                    Err(e) => {
                        println!("❌ ERROR: Command failed in {:?}: {}", duration, e);
                    }
                }
                
                result
            }};
        }
    };
    
    let final_code = quote! {
        #audit_macro
        
        #syntax_tree
    };
    
    Ok(final_code.to_string())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <input_file> <output_file>", args[0]);
        return Ok(());
    }
    
    let input_file = Path::new(&args[1]);
    let output_file = Path::new(&args[2]);
    
    println!("🔧 Applying process audit transformation...");
    println!("   Input: {:?}", input_file);
    println!("   Output: {:?}", output_file);
    
    let transformed = transform_file(input_file)?;
    #[syscall="write"]
    std::fs::write(output_file, transformed)?;
    
    println!("✅ Process audit transformation complete!");
    println!("   All process executions will now be audited");
    
    Ok(())
}
