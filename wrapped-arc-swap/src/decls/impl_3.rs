macro_rules! deps {
    () => {
        Access!();
        DynGuard!();
        DynAccess!();
        Guard!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl < T > Access < T > for dyn DynAccess < T > + '_ { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . load () } }
    };
}

impl_3!();