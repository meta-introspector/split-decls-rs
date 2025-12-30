// Generated macro for Yaml (enum)
macro_rules! Depcrate_content_yaml_vendored_yamlYaml {
() => {
// Module: crate::content::yaml::vendored::yaml
// Provides: {"Yaml"}
// Dependencies: {}
# [doc = " A YAML node is stored as this `Yaml` enumeration, which provides an easy way to"] # [doc = " access your YAML document."] # [derive (Clone , PartialEq , PartialOrd , Debug , Eq , Ord , Hash)] pub enum Yaml { # [doc = " Float types are stored as [`String`] and parsed on demand."] # [doc = " Note that [`f64'] does NOT implement [`Eq'] trait and can NOT be stored in [`BTreeMap`]."] Real (string :: String) , # [doc = " YAML int is stored as i64."] Integer (i64) , # [doc = " YAML scalar."] String (string :: String) , # [doc = " YAML bool, e.g. `true` or `false`."] Boolean (bool) , # [doc = " YAML array, can be accessed as a `Vec`."] Array (self :: Array) , # [doc = " YAML hash, can be accessed as a sorted vector of key/value pairs."] # [doc = ""] # [doc = " Insertion order will match the order of insertion into the map."] Hash (self :: Hash) , # [doc = " YAML null, e.g. `null` or `~`."] Null , # [doc = " Accessing a nonexistent node via the Index trait returns `BadValue`. This"] # [doc = " simplifies error handling in the calling code. Invalid type conversion also"] # [doc = " returns `BadValue`."] BadValue , }
};
}
