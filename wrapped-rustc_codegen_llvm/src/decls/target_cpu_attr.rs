macro_rules! deps {
    () => {
        CreateAttrStringValue!();
        CodegenCx!();
    };
}

macro_rules! target_cpu_attr {
    () => {
        deps!();
        pub (crate) fn target_cpu_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> & 'll Attribute { let target_cpu = llvm_util :: target_cpu (cx . tcx . sess) ; llvm :: CreateAttrStringValue (cx . llcx , "target-cpu" , target_cpu) }
    };
}

target_cpu_attr!();