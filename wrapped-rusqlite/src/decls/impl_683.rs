macro_rules! deps {
    () => {
        Result!();
        Name!();
        Named!();
    };
}

macro_rules! impl_683 {
    () => {
        deps!();
        impl Name for & CStr { # [inline] fn as_cstr (& self) -> Result < Named < '_ > > { Ok (Named :: C (self)) } }
    };
}

impl_683!()