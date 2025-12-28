macro_rules! cfg_no_atomic_cas {
    () => {
        # [macro_export] macro_rules ! cfg_no_atomic_cas { ($ ($ tt : tt) *) => { $ ($ tt) * } ; }
    };
}

cfg_no_atomic_cas!();