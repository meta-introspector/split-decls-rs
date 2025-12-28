macro_rules! deps {
    () => {
        BoundedSenderInner!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T > Drop for BoundedSenderInner < T > { fn drop (& mut self) { let prev = self . inner . num_senders . fetch_sub (1 , SeqCst) ; if prev == 1 { self . close_channel () ; } } }
    };
}

impl_74!()