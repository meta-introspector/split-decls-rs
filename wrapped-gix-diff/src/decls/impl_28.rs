macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl State { fn clear (& mut self) { self . trees . clear () ; self . buf1 . clear () ; self . buf2 . clear () ; self . change_id = 0 ; } }
    };
}

impl_28!()