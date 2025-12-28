macro_rules! deps {
    () => {
        AbortHandle!();
        AbortRegistration!();
        AbortInner!();
    };
}

macro_rules! impl_1338 {
    () => {
        deps!();
        impl AbortHandle { # [doc = " Creates an (`AbortHandle`, `AbortRegistration`) pair which can be used"] # [doc = " to abort a running future or stream."] # [doc = ""] # [doc = " This function is usually paired with a call to [`Abortable::new`]."] pub fn new_pair () -> (Self , AbortRegistration) { let inner = Arc :: new (AbortInner { waker : AtomicWaker :: new () , aborted : AtomicBool :: new (false) }) ; (Self { inner : inner . clone () } , AbortRegistration { inner }) } }
    };
}

impl_1338!();