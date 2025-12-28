macro_rules! deps {
    () => {
        TypeName!();
        TypeMap!();
        Type!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl std :: ops :: Deref for TypeMap { type Target = HashMap < TypeName , HashSet < Type > > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_223!()