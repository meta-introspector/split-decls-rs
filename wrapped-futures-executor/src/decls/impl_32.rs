macro_rules! deps {
    () => {
        ThreadPool!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Clone for ThreadPool { fn clone (& self) -> Self { self . state . cnt . fetch_add (1 , Ordering :: Relaxed) ; Self { state : self . state . clone () } } }
    };
}

impl_32!()