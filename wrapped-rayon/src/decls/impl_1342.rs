macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_1342 {
    () => {
        deps!();
        impl < 'a > Drop for Drain < 'a > { fn drop (& mut self) { self . string . drain (self . range . clone ()) ; } }
    };
}

impl_1342!()