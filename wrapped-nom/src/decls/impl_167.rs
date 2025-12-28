macro_rules! deps {
    () => {
        Needed!();
        Err!();
        IsStreaming!();
        Streaming!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl IsStreaming for Streaming { fn incomplete < E , F : FnOnce () -> E > (needed : Needed , _err_f : F) -> Err < E > { Err :: Incomplete (needed) } # [inline] fn is_streaming () -> bool { true } }
    };
}

impl_167!()