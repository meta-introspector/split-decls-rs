macro_rules! deps {
    () => {
        FileSnapshot!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < T : std :: fmt :: Debug > std :: ops :: DerefMut for FileSnapshot < T > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . value } }
    };
}

impl_14!()