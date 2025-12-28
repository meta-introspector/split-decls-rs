macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl From < Bytes > for Vec < u8 > { fn from (bytes : Bytes) -> Vec < u8 > { let bytes = ManuallyDrop :: new (bytes) ; unsafe { (bytes . vtable . into_vec) (& bytes . data , bytes . ptr , bytes . len) } } }
    };
}

impl_113!();