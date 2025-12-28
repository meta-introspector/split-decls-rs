macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Drop for Item { fn drop (& mut self) { self . tree . remove (& self . key) ; } }
    };
}

impl_5!();