macro_rules! force_eval {
    () => {
        macro_rules ! force_eval { ($ e : expr) => { unsafe { :: core :: ptr :: read_volatile (&$ e) } } ; }
    };
}

force_eval!();