macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl std :: ops :: Deref for State < '_ > { type Target = pp :: Printer ; fn deref (& self) -> & Self :: Target { & self . s } }
    };
}

impl_7!()