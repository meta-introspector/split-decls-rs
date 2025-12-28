macro_rules! deps {
    () => {
        Func!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Drop for Func { fn drop (& mut self) { if ! self . data . is_null () { (self . drop) (self . data) ; } unsafe { * self . offset -= self . size } ; } }
    };
}

impl_56!();