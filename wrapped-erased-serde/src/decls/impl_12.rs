macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Drop for Any { fn drop (& mut self) { unsafe { (self . drop) (& mut self . value) } } }
    };
}

impl_12!();