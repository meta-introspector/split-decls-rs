macro_rules! deps {
    () => {
        CollectedSizednessBounds!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl CollectedSizednessBounds { # [doc = " Returns `true` if any of `Trait`, `?Trait` or `!Trait` were encountered for `Sized`,"] # [doc = " `MetaSized` or `PointeeSized`."] fn any (& self) -> bool { self . sized . any () || self . meta_sized . any () || self . pointee_sized . any () } }
    };
}

impl_429!()