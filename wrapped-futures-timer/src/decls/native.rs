macro_rules! native {
    () => {
        # [cfg (not (all (target_arch = "wasm32" , feature = "wasm-bindgen")))] mod native ;
    };
}

native!()