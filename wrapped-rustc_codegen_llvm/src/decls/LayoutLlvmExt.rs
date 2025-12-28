macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! LayoutLlvmExt {
    () => {
        deps!();
        pub (crate) trait LayoutLlvmExt < 'tcx > { fn is_llvm_immediate (& self) -> bool ; fn is_llvm_scalar_pair (& self) -> bool ; fn llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type ; fn immediate_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type ; fn scalar_llvm_type_at < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , scalar : Scalar) -> & 'a Type ; fn scalar_pair_element_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , index : usize , immediate : bool ,) -> & 'a Type ; }
    };
}

LayoutLlvmExt!()