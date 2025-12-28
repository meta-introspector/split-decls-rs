macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T > FusedFuture for Receiver < T > { fn is_terminated (& self) -> bool { if self . inner . complete . load (SeqCst) { if let Some (slot) = self . inner . data . try_lock () { if slot . is_some () { return false ; } } true } else { false } } }
    };
}

impl_119!();