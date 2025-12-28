macro_rules! deps {
    () => {
        DLLStorageClass!();
    };
}

macro_rules! set_dllimport_storage_class {
    () => {
        deps!();
        pub (crate) fn set_dllimport_storage_class < 'll > (v : & 'll Value) { unsafe { LLVMSetDLLStorageClass (v , DLLStorageClass :: DllImport) ; } }
    };
}

set_dllimport_storage_class!()