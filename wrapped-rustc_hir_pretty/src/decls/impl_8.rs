macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl std :: ops :: DerefMut for State < '_ > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . s } }
    };
}

impl_8!()