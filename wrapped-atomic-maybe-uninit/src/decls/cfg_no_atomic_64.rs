macro_rules! cfg_no_atomic_64 {
    () => {
        # [macro_export] macro_rules ! cfg_no_atomic_64 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; }
    };
}

cfg_no_atomic_64!();