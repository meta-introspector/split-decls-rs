macro_rules! deps {
    () => {
        Element!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        impl < 'a > Drop for Element < 'a > { fn drop (& mut self) { self . 0 . fetch_add (1 , Ordering :: SeqCst) ; } }
    };
}

impl_371!()