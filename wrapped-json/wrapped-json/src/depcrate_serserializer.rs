// Generated macro for Serializer (struct)
macro_rules! Depcrate_serSerializer {
() => {
// Module: crate::ser
// Provides: {"Serializer"}
// Dependencies: {}
# [doc = " A structure for serializing Rust values into JSON."] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub struct Serializer < W , F = CompactFormatter > { writer : W , formatter : F , }
};
}
