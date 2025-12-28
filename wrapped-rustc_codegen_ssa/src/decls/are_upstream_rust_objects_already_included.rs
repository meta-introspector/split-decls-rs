macro_rules! are_upstream_rust_objects_already_included {
    () => {
        pub (crate) fn are_upstream_rust_objects_already_included (sess : & Session) -> bool { match sess . lto () { config :: Lto :: Fat => true , config :: Lto :: Thin => { ! sess . opts . cg . linker_plugin_lto . enabled () } config :: Lto :: No | config :: Lto :: ThinLocal => false , } }
    };
}

are_upstream_rust_objects_already_included!();