use crate::track_transform;

/// Add compiler bootstrap feature flags to lib.rs files
pub fn add_bootstrap_features(content: &str) -> String {
    track_transform!(content, "BOOTSTRAP_FEATURES", "Add compiler bootstrap feature flags", |content: &str| {
        let bootstrap_features = vec![
            "rustc_private",
            "staged_api", 
            "if_let_guard",
            "core_io_borrowed_buf",
            "array_windows",
        ];
        
        let mut result = content.to_string();
        for feature in bootstrap_features {
            let feature_line = format!("#![feature({})]", feature);
            if !result.contains(&feature_line) {
                result = format!("{}\n{}", feature_line, result);
            }
        }
        result
    })
}
