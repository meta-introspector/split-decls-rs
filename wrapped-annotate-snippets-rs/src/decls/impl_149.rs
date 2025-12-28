macro_rules! deps {
    () => {
        OptionCow!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 'a > From < & 'a String > for OptionCow < 'a > { fn from (value : & 'a String) -> Self { Self (Some (Cow :: Borrowed (value . as_str ()))) } }
    };
}

impl_149!()