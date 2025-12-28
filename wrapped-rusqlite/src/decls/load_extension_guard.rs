macro_rules! load_extension_guard {
    () => {
        # [cfg (feature = "load_extension")] mod load_extension_guard ;
    };
}

load_extension_guard!()