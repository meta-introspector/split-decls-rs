macro_rules! Either {
    () => {
        pub (crate) enum Either { IndexPath (PathBuf) , MultiIndexFile (Arc < gix_pack :: multi_index :: File >) , }
    };
}

Either!()