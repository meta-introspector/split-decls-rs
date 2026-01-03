/// Common preprocessing functions for rustc source files
/// Fixes syntax issues that prevent compilation when files are included

/// Apply all preprocessing filters to fix syntax issues
pub fn preprocess_content(content: &str) -> String {
    content
        .replace("//!", "//")  // Fix inner doc comments
        .replace("/*!", "/*")  // Fix inner block doc comments
        .replace("#![", "#[")  // Fix inner attributes
}

/// Apply preprocessing and return processed content as String
pub fn preprocess_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(preprocess_content(&content))
}
