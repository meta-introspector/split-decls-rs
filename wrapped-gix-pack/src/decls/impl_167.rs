macro_rules! deps {
    () => {
        PackLocation!();
        Location!();
    };
}

macro_rules! impl_167 {
    () => {
        deps!();
        impl PackLocation { # [doc = " Directly go through to `LookedUp` variant, panic otherwise"] pub fn is_none (& self) -> bool { match self { PackLocation :: LookedUp (opt) => opt . is_none () , PackLocation :: NotLookedUp => unreachable ! ("must have been resolved") , } } # [doc = " Directly go through to `LookedUp` variant, panic otherwise"] pub fn as_ref (& self) -> Option < & crate :: data :: entry :: Location > { match self { PackLocation :: LookedUp (opt) => opt . as_ref () , PackLocation :: NotLookedUp => unreachable ! ("must have been resolved") , } } }
    };
}

impl_167!()