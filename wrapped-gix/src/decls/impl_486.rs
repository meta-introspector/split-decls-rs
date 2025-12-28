macro_rules! deps {
    () => {
        PathCursor!();
    };
}

macro_rules! impl_486 {
    () => {
        deps!();
        impl Drop for PathCursor < '_ > { fn drop (& mut self) { self . 0 . pop () ; } }
    };
}

impl_486!();