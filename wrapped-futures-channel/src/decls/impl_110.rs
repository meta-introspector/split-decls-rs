macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < T > Drop for Sender < T > { fn drop (& mut self) { self . inner . drop_tx () } }
    };
}

impl_110!()