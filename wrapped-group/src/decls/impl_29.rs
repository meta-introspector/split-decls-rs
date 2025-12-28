macro_rules! deps {
    () => {
        Wnaf!();
        Group!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a , G : Group > Wnaf < usize , & 'a [G] , & 'a mut Vec < i64 > > { # [doc = " Constructs new space for the scalar representation while borrowing"] # [doc = " the computed window table, for sending the window table across threads."] pub fn shared (& self) -> Wnaf < usize , & 'a [G] , Vec < i64 > > { Wnaf { base : self . base , scalar : vec ! [] , window_size : self . window_size , } } }
    };
}

impl_29!();