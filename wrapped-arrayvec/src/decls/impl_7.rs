macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < const CAP : usize > Default for ArrayString < CAP > { # [doc = " Return an empty `ArrayString`"] fn default () -> ArrayString < CAP > { ArrayString :: new () } }
    };
}

impl_7!();