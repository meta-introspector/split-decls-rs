macro_rules! deps {
    () => {
        GeP3!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl Add < GeP3 > for GeP3 { type Output = GeP3 ; fn add (self , other : GeP3) -> GeP3 { (self + other . to_cached ()) . to_p3 () } }
    };
}

impl_108!()