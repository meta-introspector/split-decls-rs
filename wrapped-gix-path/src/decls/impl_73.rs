macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl RelativePath { fn new_unchecked (value : & BStr) -> Result < & RelativePath , Error > { # [allow (unsafe_code)] unsafe { std :: mem :: transmute (value) } } }
    };
}

impl_73!()