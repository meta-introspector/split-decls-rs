use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn process_refactor_config(config_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(config_path)?;
    let config: RefactorConfig = toml::from_str(&config_str)?;
    fs::create_dir_all(&config.export.target_dir)?;
    for (i, rule) in config.splits.iter().enumerate() {
        let output = Command::new("rg")
            .args(&["-A", "5", &rule.pattern, ".", "--type", "rust"])
            .output()?;
        let matches = String::from_utf8_lossy(&output.stdout);
        let wrapped_code = wrap_code_with_rule(&matches, rule);
        let file_path = format!("{}/split_{}.rs", config.export.target_dir, i);
        fs::write(file_path, wrapped_code)?;
    }
    if let Some(decl_config) = config.decl_refactoring.as_ref() {
        perform_decl_refactoring(decl_config)?;
    }
    Ok(())
}
