macro_rules! wasm {
    () => {
        # [cfg (all (target_arch = "wasm32" , feature = "wasm-bindgen"))] mod wasm ;
    };
}

wasm!();