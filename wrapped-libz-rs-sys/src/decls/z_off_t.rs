macro_rules! z_off_t {
    () => {
        # [cfg (target_arch = "wasm32")] pub type z_off_t = i64 ;
    };
}

z_off_t!()