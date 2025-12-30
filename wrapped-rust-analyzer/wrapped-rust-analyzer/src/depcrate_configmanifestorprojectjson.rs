// Generated macro for ManifestOrProjectJson (enum)
macro_rules! Depcrate_configManifestOrProjectJson {
() => {
// Module: crate::config
// Provides: {"ManifestOrProjectJson"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Debug , Clone , PartialEq , Eq)] # [serde (untagged)] enum ManifestOrProjectJson { Manifest (Utf8PathBuf) , ProjectJson (ProjectJsonData) , DiscoveredProjectJson { data : ProjectJsonData , # [serde (serialize_with = "serialize_abs_pathbuf")] # [serde (deserialize_with = "deserialize_abs_pathbuf")] buildfile : AbsPathBuf , } , }
};
}
