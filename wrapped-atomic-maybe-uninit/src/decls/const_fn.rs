macro_rules! const_fn {
    () => {
        # [doc = " Make the given function const if the given condition is true."] macro_rules ! const_fn { (const_if : # [cfg ($ ($ cfg : tt) +)] ; $ (# [$ ($ attr : tt) *]) * $ vis : vis const $ ($ rest : tt) *) => { # [cfg ($ ($ cfg) +)] $ (# [$ ($ attr) *]) * $ vis const $ ($ rest) * # [cfg (not ($ ($ cfg) +))] $ (# [$ ($ attr) *]) * $ vis $ ($ rest) * } ; }
    };
}

const_fn!()