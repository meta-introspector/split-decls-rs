macro_rules! deps {
    () => {
        Boolean!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Boolean { # [doc = " Return true if the boolean is a true value."] # [doc = ""] # [doc = " Note that the inner value is accessible directly as well."] pub fn is_true (self) -> bool { self . 0 } }
    };
}

impl_5!()