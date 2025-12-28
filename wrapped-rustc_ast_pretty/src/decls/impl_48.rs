macro_rules! deps {
    () => {
        Printer!();
        State!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl std :: ops :: Deref for State < '_ > { type Target = pp :: Printer ; fn deref (& self) -> & Self :: Target { & self . s } }
    };
}

impl_48!()