// Generated macro for TestRegistry (struct)
macro_rules! Depcrate_registryTestRegistry {
() => {
// Module: crate::registry
// Provides: {"TestRegistry"}
// Dependencies: {}
# [doc = " A local registry fixture"] # [doc = ""] # [doc = " Most tests won't need to call this directly but instead interact with [`Package`]"] pub struct TestRegistry { server : Option < HttpServerHandle > , index_url : Url , path : PathBuf , api_url : Url , dl_url : Url , token : Token , }
};
}
