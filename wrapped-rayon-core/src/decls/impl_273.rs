macro_rules! deps {
    () => {
        ThreadPool!();
    };
}

macro_rules! impl_273 {
    () => {
        deps!();
        impl Drop for ThreadPool { fn drop (& mut self) { self . registry . terminate () ; } }
    };
}

impl_273!()