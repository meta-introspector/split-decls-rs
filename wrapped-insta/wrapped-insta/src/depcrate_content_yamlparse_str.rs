// Generated macro for parse_str (function)
macro_rules! Depcrate_content_yamlparse_str {
() => {
// Module: crate::content::yaml
// Provides: {"parse_str"}
// Dependencies: {}
pub fn parse_str (s : & str , filename : & Path) -> Result < Content , Error > { let mut blobs = crate :: content :: yaml :: vendored :: yaml :: YamlLoader :: load_from_str (s) . map_err (| _ | Error :: FailedParsingYaml (filename . to_path_buf ())) ? ; match (blobs . pop () , blobs . pop ()) { (Some (blob) , None) => from_yaml_blob (blob , filename) , _ => Err (Error :: FailedParsingYaml (filename . to_path_buf ())) , } }
};
}
