macro_rules! wasm32 {
    () => {
        # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] pub mod wasm32 ;
    };
}

wasm32!();