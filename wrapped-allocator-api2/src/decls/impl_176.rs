macro_rules! deps {
    () => {
        Vec!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < T > Default for Vec < T > { # [doc = " Creates an empty `Vec<T>`."] # [doc = ""] # [doc = " The vector will not allocate until elements are pushed onto it."] # [inline (always)] fn default () -> Vec < T > { Vec :: new () } }
    };
}

impl_176!()