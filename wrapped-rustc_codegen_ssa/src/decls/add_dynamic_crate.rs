macro_rules! deps {
    () => {
        Linker!();
    };
}

macro_rules! add_dynamic_crate {
    () => {
        deps!();
        fn add_dynamic_crate (cmd : & mut dyn Linker , sess : & Session , cratepath : & Path) { cmd . link_dylib_by_path (& rehome_lib_path (sess , cratepath) , true) ; }
    };
}

add_dynamic_crate!()