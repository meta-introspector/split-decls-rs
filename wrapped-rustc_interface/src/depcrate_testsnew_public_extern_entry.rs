// Generated macro for new_public_extern_entry (function)
macro_rules! Depcrate_testsnew_public_extern_entry {
() => {
// Module: crate::tests
// Provides: {"new_public_extern_entry"}
// Dependencies: {}
fn new_public_extern_entry < S , I > (locations : I) -> ExternEntry where S : Into < String > , I : IntoIterator < Item = S > , { let locations = locations . into_iter () . map (| s | CanonicalizedPath :: new (PathBuf :: from (s . into ()))) . collect () ; ExternEntry { location : ExternLocation :: ExactPaths (locations) , is_private_dep : false , add_prelude : true , nounused_dep : false , force : false , } }
};
}
