// Generated macro for Metadata (struct)
macro_rules! DepcrateMetadata {
() => {
// Module: crate
// Provides: {"Metadata"}
// Dependencies: {}
# [doc = " Describes a tree of metadata for our filesystem tree"] # [doc = ""] # [doc = " Must match the JSON emitted by the `CollectLicenseMetadata` bootstrap tool."] # [derive (serde :: Deserialize , Clone , Debug , PartialEq , Eq)] struct Metadata { files : Node , }
};
}
