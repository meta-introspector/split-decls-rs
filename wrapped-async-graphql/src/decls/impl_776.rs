macro_rules! deps {
    () => {
        Json!();
    };
}

macro_rules! impl_776 {
    () => {
        deps!();
        impl < T > DerefMut for Json < T > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } }
    };
}

impl_776!()