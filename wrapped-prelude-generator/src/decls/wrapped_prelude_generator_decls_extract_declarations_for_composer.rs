use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub async fn extract_declarations_for_composer(
    file_path: &std::path::Path,
    rustc_info: &RustcInfo,
    crate_name: &str,
    verbose: u8,
    warnings: &mut Vec<String>,
    canonical_output_root: &std::path::Path,
) -> anyhow::Result<crate::types::AllDeclarationsExtractionResult> {
    let extraction_result = crate::declaration_processing::extract_all_declarations_from_file(
            file_path,
            &std::path::PathBuf::new(),
            false,
            verbose,
            &rustc_info,
            crate_name,
            warnings,
            canonical_output_root,
        )
        .await?;
    Ok(extraction_result)
}
