pub fn test_function() -> Result<String> {
    let config = SplitDeclsConfig {
        test_field: "Hello from included file!".to_string(),
    };
    
    Ok(format!("Config: {:?}", config))
}
