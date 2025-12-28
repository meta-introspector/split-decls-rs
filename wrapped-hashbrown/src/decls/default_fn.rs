macro_rules! default_fn {
    () => {
        # [cfg (not (feature = "nightly"))] macro_rules ! default_fn { ($ ($ tt : tt) *) => { $ ($ tt) * } }
    };
}

default_fn!()