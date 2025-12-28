macro_rules! deps {
    () => {
        Handle!();
        Proxy!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl From < crate :: Handle > for Proxy < crate :: Handle > { fn from (odb : crate :: Handle) -> Self { let object_hash = odb . store . object_hash ; Proxy :: new (odb , object_hash) } }
    };
}

impl_151!();