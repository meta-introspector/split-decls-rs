macro_rules! deps {
    () => {
        CollectedBound!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl CollectedBound { # [doc = " Returns `true` if any of `Trait`, `?Trait` or `!Trait` were encountered."] fn any (& self) -> bool { self . positive || self . maybe || self . negative } }
    };
}

impl_427!();