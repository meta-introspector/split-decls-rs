macro_rules! if_wasm {
    () => {
        macro_rules ! if_wasm { ($ ($ item : item) *) => { $ (# [cfg (target_arch = "wasm32")] $ item) * } }
    };
}

if_wasm!();