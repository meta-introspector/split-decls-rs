macro_rules! deps {
    () => {
        NewDir!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl Drop for NewDir < '_ > { fn drop (& mut self) { self . 0 . pop () ; } }
    };
}

impl_485!()