macro_rules! deps {
    () => {
        IsStreaming!();
        Complete!();
        Needed!();
        Err!();
        Error!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl IsStreaming for Complete { fn incomplete < E , F : FnOnce () -> E > (_needed : Needed , err_f : F) -> Err < E > { Err :: Error (err_f ()) } # [inline] fn is_streaming () -> bool { false } }
    };
}

impl_169!()