macro_rules! deps {
    () => {
        Current!();
        CompatSink!();
    };
}

macro_rules! with_sink_context {
    () => {
        deps!();
        # [cfg (feature = "sink")] fn with_sink_context < T , Item , R , F > (compat : & mut CompatSink < T , Item > , f : F) -> R where T : Unpin , F : FnOnce (Pin < & mut T > , & mut Context < '_ >) -> R , { let current = Current :: new () ; let waker = current . as_waker () ; let mut cx = Context :: from_waker (& waker) ; f (Pin :: new (& mut compat . inner) , & mut cx) }
    };
}

with_sink_context!();