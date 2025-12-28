macro_rules! deps {
    () => {
        DynAccess!();
        DynGuard!();
        Access!();
        Guard!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < T > Access < T > for dyn DynAccess < T > + '_ + Sync + Send { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . load () } }
    };
}

impl_5!();