macro_rules! deps {
    () => {
        DropCounter!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Drop for DropCounter < '_ > { fn drop (& mut self) { self . count . set (self . count . get () + 1) ; } }
    };
}

impl_36!()