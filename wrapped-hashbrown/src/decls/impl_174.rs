macro_rules! deps {
    () => {
        RawParDrain!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < T , A : Allocator > Drop for RawParDrain < '_ , T , A > { fn drop (& mut self) { unsafe { self . table . as_mut () . clear () ; } } }
    };
}

impl_174!()