// Generated macro for LayoutLlvmExt (trait)
macro_rules! Depcrate_type_ofLayoutLlvmExt {
() => {
// Module: crate::type_of
// Provides: {"LayoutLlvmExt"}
// Dependencies: {}
pub (crate) trait LayoutLlvmExt < 'tcx > { fn is_llvm_immediate (& self) -> bool ; fn is_llvm_scalar_pair (& self) -> bool ; fn llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type ; fn immediate_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx >) -> & 'a Type ; fn scalar_llvm_type_at < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , scalar : Scalar) -> & 'a Type ; fn scalar_pair_element_llvm_type < 'a > (& self , cx : & CodegenCx < 'a , 'tcx > , index : usize , immediate : bool ,) -> & 'a Type ; }
};
}
