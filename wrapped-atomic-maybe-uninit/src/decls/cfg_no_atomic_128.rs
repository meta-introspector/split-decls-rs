macro_rules! cfg_no_atomic_128 {
    () => {
        # [macro_export] macro_rules ! cfg_no_atomic_128 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; }
    };
}

cfg_no_atomic_128!()