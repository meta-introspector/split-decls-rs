macro_rules! DLLStorageClass {
    () => {
        # [doc = " LLVMDLLStorageClass"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum DLLStorageClass { # [allow (dead_code)] Default = 0 , DllImport = 1 , # [allow (dead_code)] DllExport = 2 , }
    };
}

DLLStorageClass!();