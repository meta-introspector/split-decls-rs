macro_rules! deps {
    () => {
        PrefilterConfig!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl PrefilterConfig { # [doc = " Returns true when this prefilter is set to the `None` variant."] fn is_none (& self) -> bool { matches ! (* self , PrefilterConfig :: None) } }
    };
}

impl_345!()