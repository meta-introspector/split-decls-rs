use crate::track_transform;
use crate::transform_imports::add_imports_semantic;

/// Add compiler bootstrap feature flags with semantic import management
pub fn add_bootstrap_features(content: &str) -> String {
    track_transform!(content, "BOOTSTRAP_FEATURES", "Add compiler bootstrap feature flags", |content: &str| {
        // First add required imports semantically
        let imports = [
            "rustc_index::newtype_index",
            "std::collections::HashMap",
        ];
        
        let mut result = add_imports_semantic(content, &imports);
        
        // Then add feature flags at the top
        let bootstrap_features = vec![
            "rustc_private",
            "staged_api", 
            "if_let_guard",
            "core_io_borrowed_buf",
            "array_windows",
        ];
        
        for feature in bootstrap_features {
            let feature_line = format!("#![feature({})]", feature);
            if !result.contains(&feature_line) {
                result = format!("{}\n{}", feature_line, result);
            }
        }
        
        result
    })
}
