macro_rules! deps {
    () => {
        AccessConvert!();
        DynGuard!();
        Access!();
        DynAccess!();
        Guard!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T , D > Access < T > for AccessConvert < D > where D : Deref , D :: Target : DynAccess < T > , { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . 0 . load () } }
    };
}

impl_17!()