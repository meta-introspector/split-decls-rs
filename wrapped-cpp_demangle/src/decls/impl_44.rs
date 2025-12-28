macro_rules! deps {
    () => {
        AutoDemangleContextInnerBarrier!();
        DemangleWrite!();
        DemangleContext!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < 'ctx , 'a , W > ops :: Deref for AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { type Target = DemangleContext < 'a , W > ; fn deref (& self) -> & Self :: Target { self . ctx } }
    };
}

impl_44!();