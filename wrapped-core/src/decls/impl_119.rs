macro_rules! deps {
    () => {
        ComObjectInner!();
        ComObject!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T : ComObjectInner + PartialEq > PartialEq for ComObject < T > { fn eq (& self , other : & Self) -> bool { let inner_self : & T = self . get () ; let other_self : & T = other . get () ; inner_self == other_self } }
    };
}

impl_119!();