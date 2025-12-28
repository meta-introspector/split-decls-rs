macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl std :: ops :: Deref for Path < '_ > { type Target = BStr ; fn deref (& self) -> & Self :: Target { self . value . as_ref () } }
    };
}

impl_42!();