macro_rules! deps {
    () => {
        OpaqueTypeStorageEntries!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl rustc_type_ir :: inherent :: OpaqueTypeStorageEntries for OpaqueTypeStorageEntries { fn needs_reevaluation (self , canonicalized : usize) -> bool { self . opaque_types != canonicalized } }
    };
}

impl_74!();