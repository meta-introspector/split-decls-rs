macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < T > Drop for Receiver < T > { fn drop (& mut self) { self . inner . drop_rx () } }
    };
}

impl_120!();