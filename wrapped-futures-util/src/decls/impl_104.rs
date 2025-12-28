macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < Fut > Drop for Shared < Fut > where Fut : Future , { fn drop (& mut self) { if self . waker_key != NULL_WAKER_KEY { if let Some (ref inner) = self . inner { # [cfg (feature = "std")] if let Ok (mut wakers) = inner . notifier . wakers . lock () { if let Some (wakers) = wakers . as_mut () { wakers . remove (self . waker_key) ; } } # [cfg (not (feature = "std"))] if let Some (wakers) = inner . notifier . wakers . lock () . as_mut () { wakers . remove (self . waker_key) ; } } } } }
    };
}

impl_104!();