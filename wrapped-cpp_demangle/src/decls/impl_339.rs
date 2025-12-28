macro_rules! deps {
    () => {
        SubstitutionTable!();
        Substitutable!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl Deref for SubstitutionTable { type Target = [Substitutable] ; fn deref (& self) -> & Self :: Target { & self . substitutions [..] } }
    };
}

impl_339!();