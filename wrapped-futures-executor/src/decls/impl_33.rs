macro_rules! deps {
    () => {
        ThreadPool!();
        Message!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Drop for ThreadPool { fn drop (& mut self) { if self . state . cnt . fetch_sub (1 , Ordering :: Relaxed) == 1 { for _ in 0 .. self . state . size { self . state . send (Message :: Close) ; } } } }
    };
}

impl_33!();