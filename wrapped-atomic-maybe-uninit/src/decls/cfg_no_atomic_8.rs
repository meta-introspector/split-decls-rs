macro_rules! cfg_no_atomic_8 {
    () => {
        # [macro_export] macro_rules ! cfg_no_atomic_8 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; }
    };
}

cfg_no_atomic_8!();