macro_rules! deps {
    () => {
        AwokenCount!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl AwokenCount { # [doc = " Get the current count."] pub fn get (& self) -> usize { self . inner . count . load (Ordering :: SeqCst) } }
    };
}

impl_36!();