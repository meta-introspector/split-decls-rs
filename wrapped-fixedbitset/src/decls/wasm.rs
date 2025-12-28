macro_rules! wasm {
    () => {
        # [cfg (all (target_family = "wasm" , target_feature = "simd128"))] mod wasm ;
    };
}

wasm!();