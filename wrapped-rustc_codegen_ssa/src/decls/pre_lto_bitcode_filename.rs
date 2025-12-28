macro_rules! pre_lto_bitcode_filename {
    () => {
        fn pre_lto_bitcode_filename (module_name : & str) -> String { format ! ("{module_name}.{PRE_LTO_BC_EXT}") }
    };
}

pre_lto_bitcode_filename!()