macro_rules! deps {
    () => {
        NameRef!();
        Name!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a > Name { # [doc = " Provide our ref-type."] pub fn as_ref (& 'a self) -> NameRef < 'a > { NameRef (self . 0 . as_ref ()) } # [doc = " Return the inner `str`."] pub fn as_str (& self) -> & str { self . 0 . as_str () } }
    };
}

impl_8!()