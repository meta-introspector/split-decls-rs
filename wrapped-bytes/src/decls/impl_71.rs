macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Clone for Bytes { # [inline] fn clone (& self) -> Bytes { unsafe { (self . vtable . clone) (& self . data , self . ptr , self . len) } } }
    };
}

impl_71!();