macro_rules! deps {
    () => {
        AutoParseDemangle!();
        DemangleWrite!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < 'a , 'b , W : 'a + DemangleWrite > ops :: DerefMut for AutoParseDemangle < 'a , 'b , W > { fn deref_mut (& mut self) -> & mut Self :: Target { self . 0 } }
    };
}

impl_37!()