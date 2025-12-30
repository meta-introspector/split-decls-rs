// Generated macro for LayoutGccExt (trait)
macro_rules! Depcrate_type_ofLayoutGccExt {
() => {
// Module: crate::type_of
// Provides: {"LayoutGccExt"}
// Dependencies: {}
pub trait LayoutGccExt < 'tcx > { fn is_gcc_immediate (& self) -> bool ; fn is_gcc_scalar_pair (& self) -> bool ; fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; fn immediate_gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; fn scalar_gcc_type_at < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , scalar : & abi :: Scalar , offset : Size ,) -> Type < 'gcc > ; fn scalar_pair_element_gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , index : usize ,) -> Type < 'gcc > ; fn pointee_info_at < 'gcc > (& self , cx : & CodegenCx < 'gcc , 'tcx > , offset : Size ,) -> Option < PointeeInfo > ; }
};
}
