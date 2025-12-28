macro_rules! deps {
    () => {
        AnyEq!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < T : Any + PartialEq > AnyEq for T { fn equals (& self , other : & dyn Any) -> bool { other . downcast_ref :: < Self > () == Some (self) } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_95!();