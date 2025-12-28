macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < T > Drop for Drain < '_ , T > { fn drop (& mut self) { if ! self . heap . is_empty () { self . heap . drain () ; } } }
    };
}

impl_36!();