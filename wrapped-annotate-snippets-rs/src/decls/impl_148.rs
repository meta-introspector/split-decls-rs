macro_rules! deps {
    () => {
        OptionCow!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < 'a > From < String > for OptionCow < 'a > { fn from (value : String) -> Self { Self (Some (Cow :: Owned (value))) } }
    };
}

impl_148!()