macro_rules! ThinLTOModule {
    () => {
        # [doc = " LLVMRustThinLTOModule"] # [repr (C)] pub (crate) struct ThinLTOModule { pub identifier : * const c_char , pub data : * const u8 , pub len : usize , }
    };
}

ThinLTOModule!()