macro_rules! cfg_no_atomic_16 {
    () => {
        # [macro_export] macro_rules ! cfg_no_atomic_16 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; }
    };
}

cfg_no_atomic_16!();