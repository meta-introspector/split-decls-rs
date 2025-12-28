macro_rules! RelocModel {
    () => {
        # [doc = " LLVMRustRelocModel"] # [derive (Copy , Clone , PartialEq)] # [repr (C)] pub (crate) enum RelocModel { Static , PIC , DynamicNoPic , ROPI , RWPI , ROPI_RWPI , }
    };
}

RelocModel!()