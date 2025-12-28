macro_rules! wasm32_simd {
    () => {
        # [cfg (blake3_wasm32_simd)] # [path = "wasm32_simd.rs"] mod wasm32_simd ;
    };
}

wasm32_simd!();