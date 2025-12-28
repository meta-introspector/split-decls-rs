macro_rules! deps {
    () => {
        Storage!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl DerefMut for Storage { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_163!()