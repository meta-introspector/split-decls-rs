macro_rules! deps {
    () => {
        LocalSender!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < T > Drop for LocalSender < T > { fn drop (& mut self) { let mut channel = self . channel . borrow_mut () ; channel . closed = true ; let _ = channel . waker . take () . map (Waker :: wake) ; } }
    };
}

impl_121!();