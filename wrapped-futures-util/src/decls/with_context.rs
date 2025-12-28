macro_rules! deps {
    () => {
        Current!();
        Compat!();
    };
}

macro_rules! with_context {
    () => {
        deps!();
        fn with_context < T , R , F > (compat : & mut Compat < T > , f : F) -> R where T : Unpin , F : FnOnce (Pin < & mut T > , & mut Context < '_ >) -> R , { let current = Current :: new () ; let waker = current . as_waker () ; let mut cx = Context :: from_waker (& waker) ; f (Pin :: new (& mut compat . inner) , & mut cx) }
    };
}

with_context!();