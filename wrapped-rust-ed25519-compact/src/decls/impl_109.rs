macro_rules! deps {
    () => {
        GeP3!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl Sub < GeP3 > for GeP3 { type Output = GeP3 ; fn sub (self , other : GeP3) -> GeP3 { (self - other . to_cached ()) . to_p3 () } }
    };
}

impl_109!()