macro_rules! deps {
    () => {
        AbortHandle!();
        AbortRegistration!();
    };
}

macro_rules! impl_1346 {
    () => {
        deps!();
        impl AbortHandle { # [doc = " Abort the `Abortable` stream/future associated with this handle."] # [doc = ""] # [doc = " Notifies the Abortable task associated with this handle that it"] # [doc = " should abort. Note that if the task is currently being polled on"] # [doc = " another thread, it will not immediately stop running. Instead, it will"] # [doc = " continue to run until its poll method returns."] pub fn abort (& self) { self . inner . aborted . store (true , Ordering :: Relaxed) ; self . inner . waker . wake () ; } # [doc = " Checks whether [`AbortHandle::abort`] was *called* on any associated"] # [doc = " [`AbortHandle`]s, which includes all the [`AbortHandle`]s linked with"] # [doc = " the same [`AbortRegistration`]. This means that it will return `true`"] # [doc = " even if:"] # [doc = " * `abort` was called after the task had completed."] # [doc = " * `abort` was called while the task was being polled - the task may still be running and"] # [doc = "   will not be stopped until `poll` returns."] # [doc = ""] # [doc = " This operation has a Relaxed ordering."] pub fn is_aborted (& self) -> bool { self . inner . aborted . load (Ordering :: Relaxed) } }
    };
}

impl_1346!()