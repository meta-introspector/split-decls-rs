macro_rules! rustc_entry {
    () => {
        # [cfg (feature = "rustc-internal-api")] mod rustc_entry ;
    };
}

rustc_entry!();