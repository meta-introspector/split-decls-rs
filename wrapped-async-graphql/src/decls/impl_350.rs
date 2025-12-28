macro_rules! deps {
    () => {
        Any!();
        Result!();
        ContextBase!();
        DataContext!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < 'a , T > DataContext < 'a > for ContextBase < 'a , T > { fn data < D : Any + Send + Sync > (& self) -> Result < & 'a D > { ContextBase :: data :: < D > (self) } fn data_unchecked < D : Any + Send + Sync > (& self) -> & 'a D { ContextBase :: data_unchecked :: < D > (self) } fn data_opt < D : Any + Send + Sync > (& self) -> Option < & 'a D > { ContextBase :: data_opt :: < D > (self) } }
    };
}

impl_350!();