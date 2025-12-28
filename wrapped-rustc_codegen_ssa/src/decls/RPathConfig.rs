macro_rules! RPathConfig {
    () => {
        pub (super) struct RPathConfig < 'a > { pub libs : & 'a [& 'a Path] , pub out_filename : PathBuf , pub is_like_darwin : bool , pub linker_is_gnu : bool , }
    };
}

RPathConfig!()