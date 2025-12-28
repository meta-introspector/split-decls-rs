macro_rules! deps {
    () => {
        CreateAttrStringValue!();
        CodegenCx!();
    };
}

macro_rules! tune_cpu_attr {
    () => {
        deps!();
        pub (crate) fn tune_cpu_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { llvm_util :: tune_cpu (cx . tcx . sess) . map (| tune_cpu | llvm :: CreateAttrStringValue (cx . llcx , "tune-cpu" , tune_cpu)) }
    };
}

tune_cpu_attr!()