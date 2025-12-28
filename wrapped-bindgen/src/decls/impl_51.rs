macro_rules! deps {
    () => {
        Type!();
        Param!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl std :: ops :: Deref for Param { type Target = Type ; fn deref (& self) -> & Self :: Target { & self . ty } }
    };
}

impl_51!()