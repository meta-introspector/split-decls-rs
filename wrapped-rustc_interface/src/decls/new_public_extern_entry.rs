macro_rules! new_public_extern_entry {
    () => {
        fn new_public_extern_entry < S , I > (locations : I) -> ExternEntry where S : Into < String > , I : IntoIterator < Item = S > , { let locations = locations . into_iter () . map (| s | CanonicalizedPath :: new (PathBuf :: from (s . into ()))) . collect () ; ExternEntry { location : ExternLocation :: ExactPaths (locations) , is_private_dep : false , add_prelude : true , nounused_dep : false , force : false , } }
    };
}

new_public_extern_entry!()