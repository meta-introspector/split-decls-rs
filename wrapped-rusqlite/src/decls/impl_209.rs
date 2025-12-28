macro_rules! deps {
    () => {
        RawStatement!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl Drop for RawStatement { fn drop (& mut self) { self . finalize_ () ; } }
    };
}

impl_209!()