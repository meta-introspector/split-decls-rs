macro_rules! deps {
    () => {
        DynGuard!();
        Guard!();
        Access!();
        DynAccess!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T , A > DynAccess < T > for A where A : Access < T > , A :: Guard : 'static , { fn load (& self) -> DynGuard < T > { DynGuard (Box :: new (Access :: load (self))) } }
    };
}

impl_15!();