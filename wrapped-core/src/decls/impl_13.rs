macro_rules! deps {
    () => {
        ThreadData!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Drop for ThreadData { fn drop (& mut self) { NUM_THREADS . fetch_sub (1 , Ordering :: Relaxed) ; } }
    };
}

impl_13!();