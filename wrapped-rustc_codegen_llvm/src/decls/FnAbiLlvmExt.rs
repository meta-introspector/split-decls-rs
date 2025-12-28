macro_rules! deps {
    () => {
        CodegenCx!();
        Builder!();
        CallConv!();
    };
}

macro_rules! FnAbiLlvmExt {
    () => {
        deps!();
        pub (crate) trait FnAbiLlvmExt < 'll , 'tcx > { fn llvm_type (& self , cx : & CodegenCx < 'll , 'tcx >) -> & 'll Type ; fn ptr_to_llvm_type (& self , cx : & CodegenCx < 'll , 'tcx >) -> & 'll Type ; fn llvm_cconv (& self , cx : & CodegenCx < 'll , 'tcx >) -> llvm :: CallConv ; # [doc = " Apply attributes to a function declaration/definition."] fn apply_attrs_llfn (& self , cx : & CodegenCx < 'll , 'tcx > , llfn : & 'll Value , instance : Option < ty :: Instance < 'tcx > > ,) ; # [doc = " Apply attributes to a function call."] fn apply_attrs_callsite (& self , bx : & mut Builder < '_ , 'll , 'tcx > , callsite : & 'll Value) ; }
    };
}

FnAbiLlvmExt!();