macro_rules! ffi {
    () => {
        # [cfg (feature = "ffi")] # [cfg_attr (docsrs , doc (cfg (all (feature = "ffi" , hyper_unstable_ffi))))] pub mod ffi ;
    };
}

ffi!()