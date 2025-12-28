macro_rules! deps {
    () => {
        Warnings!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl std :: ops :: Deref for Warnings { type Target = Vec < String > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_332!()