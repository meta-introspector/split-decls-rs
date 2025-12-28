macro_rules! if_hyper {
    () => {
        macro_rules ! if_hyper { ($ ($ item : item) *) => { $ (# [cfg (not (target_arch = "wasm32"))] $ item) * } }
    };
}

if_hyper!();