macro_rules! deps {
    () => {
        DemangleContext!();
        DemangleWrite!();
        AutoParseDemangle!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'a , 'b , W : 'a + DemangleWrite > ops :: Deref for AutoParseDemangle < 'a , 'b , W > { type Target = DemangleContext < 'a , W > ; fn deref (& self) -> & Self :: Target { self . 0 } }
    };
}

impl_36!()