macro_rules! deps {
    () => {
        Wnaf!();
        Group!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a , G : Group > Wnaf < usize , & 'a mut Vec < G > , & 'a [i64] > { # [doc = " Constructs new space for the window table while borrowing"] # [doc = " the computed scalar representation, for sending the scalar representation"] # [doc = " across threads."] pub fn shared (& self) -> Wnaf < usize , Vec < G > , & 'a [i64] > { Wnaf { base : vec ! [] , scalar : self . scalar , window_size : self . window_size , } } }
    };
}

impl_31!();