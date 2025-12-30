// Generated macro for from_yaml_blob (function)
macro_rules! Depcrate_content_yamlfrom_yaml_blob {
() => {
// Module: crate::content::yaml
// Provides: {"from_yaml_blob"}
// Dependencies: {}
fn from_yaml_blob (blob : YamlValue , filename : & Path) -> Result < Content , Error > { match blob { YamlValue :: Null => Ok (Content :: None) , YamlValue :: Boolean (b) => Ok (Content :: from (b)) , YamlValue :: Integer (num) => Ok (Content :: from (num)) , YamlValue :: Real (real_str) => { let real : f64 = real_str . parse () . unwrap () ; Ok (Content :: from (real)) } YamlValue :: String (s) => Ok (Content :: from (s)) , YamlValue :: Array (seq) => { let seq = seq . into_iter () . map (| x | from_yaml_blob (x , filename)) . collect :: < Result < _ , Error > > () ? ; Ok (Content :: Seq (seq)) } YamlValue :: Hash (obj) => { let obj = obj . into_iter () . map (| (k , v) | Ok ((from_yaml_blob (k , filename) ? , from_yaml_blob (v , filename) ?))) . collect :: < Result < _ , Error > > () ? ; Ok (Content :: Map (obj)) } YamlValue :: BadValue => Err (Error :: FailedParsingYaml (filename . to_path_buf ())) , } }
};
}
