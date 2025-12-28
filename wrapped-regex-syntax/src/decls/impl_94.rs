macro_rules! deps {
    () => {
        ClassSetRange!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl ClassSetRange { # [doc = " Returns true if and only if this character class range is valid."] # [doc = ""] # [doc = " The only case where a range is invalid is if its start is greater than"] # [doc = " its end."] pub fn is_valid (& self) -> bool { self . start . c <= self . end . c } }
    };
}

impl_94!()