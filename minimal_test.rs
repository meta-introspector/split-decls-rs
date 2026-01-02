use syn::parse_file;
use split_decls_genesis::build_lib::add_prelude;

fn main() {
    // Minimal test case that reproduces the attribute spacing issue
    let source = r#"
#[cfg(target_os = "uefi")]
pub fn current_dir() -> io::Result<PathBuf> {
    unsupported()
}
"#;
    
    println!("Original: {:?}", parse_file(source).is_ok());
    
    let with_prelude = add_prelude(source);
    println!("With prelude: {:?}", parse_file(&with_prelude).is_ok());
    
    if parse_file(&with_prelude).is_err() {
        println!("Error: {}", parse_file(&with_prelude).unwrap_err());
        println!("Generated code:\n{}", with_prelude);
    }
}
