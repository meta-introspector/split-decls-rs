macro_rules! cfg_no_atomic_32 {
    () => {
        # [macro_export] macro_rules ! cfg_no_atomic_32 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; }
    };
}

cfg_no_atomic_32!();