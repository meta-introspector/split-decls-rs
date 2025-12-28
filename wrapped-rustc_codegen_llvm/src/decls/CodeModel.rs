macro_rules! CodeModel {
    () => {
        # [doc = " LLVMRustCodeModel"] # [derive (Copy , Clone)] # [repr (C)] pub (crate) enum CodeModel { Tiny , Small , Kernel , Medium , Large , None , }
    };
}

CodeModel!()