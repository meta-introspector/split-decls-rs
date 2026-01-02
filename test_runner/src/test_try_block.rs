use syn::parse_file;

fn test_incremental_parsing() {
    let mut content = String::new();
    
    // Add lines incrementally from translation.rs
    let lines = vec![
        "use std::borrow::Cow;",
        "use std::env;",
        "use std::error::Report;",
        "use std::sync::Arc;",
        "",
        "pub use rustc_error_messages::{FluentArgs, LazyFallbackBundle};",
        "use tracing::{debug, trace};",
        "",
        "use crate::error::{TranslateError, TranslateErrorKind};",
        "use crate::snippet::Style;",
    ];
    
    for (i, line) in lines.iter().enumerate() {
        content.push_str(line);
        content.push('\n');
        
        println!("🧪 Testing with {} lines:", i + 1);
        match parse_file(&content) {
            Ok(_) => println!("✅ Parses OK"),
            Err(e) => {
                println!("❌ Failed: {}", e);
                println!("📄 Content so far:\n{}", content);
                return;
            }
        }
    }
    
    println!("✅ All initial lines parse successfully!");
}

fn main() {
    test_incremental_parsing();
}
