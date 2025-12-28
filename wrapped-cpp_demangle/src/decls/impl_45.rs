macro_rules! deps {
    () => {
        DemangleWrite!();
        AutoDemangleContextInnerBarrier!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < 'ctx , 'a , W > ops :: DerefMut for AutoDemangleContextInnerBarrier < 'ctx , 'a , W > where W : 'a + DemangleWrite , 'a : 'ctx , { fn deref_mut (& mut self) -> & mut Self :: Target { self . ctx } }
    };
}

impl_45!();