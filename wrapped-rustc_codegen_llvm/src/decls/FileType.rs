macro_rules! FileType {
    () => {
        # [doc = " LLVMRustFileType"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum FileType { AssemblyFile , ObjectFile , }
    };
}

FileType!()