macro_rules! deps {
    () => {
        Guard!();
        DynGuard!();
        Access!();
        DynAccess!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < T > Access < T > for dyn DynAccess < T > + '_ + Send { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . load () } }
    };
}

impl_4!()