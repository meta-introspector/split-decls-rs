macro_rules! wasm {
    () => {
        # [cfg (feature = "wasm")] pub mod wasm ;
    };
}

wasm!()