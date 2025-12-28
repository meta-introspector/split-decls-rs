macro_rules! deps {
    () => {
        UnboundedSenderInner!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T > Drop for UnboundedSenderInner < T > { fn drop (& mut self) { let prev = self . inner . num_senders . fetch_sub (1 , SeqCst) ; if prev == 1 { self . close_channel () ; } } }
    };
}

impl_73!();