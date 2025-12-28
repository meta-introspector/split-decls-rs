macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Drop for Bytes { # [inline] fn drop (& mut self) { unsafe { (self . vtable . drop) (& mut self . data , self . ptr , self . len) } } }
    };
}

impl_70!();