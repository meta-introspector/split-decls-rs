// Generated macro for FnAbiGccExt (trait)
macro_rules! Depcrate_abiFnAbiGccExt {
() => {
// Module: crate::abi
// Provides: {"FnAbiGccExt"}
// Dependencies: {}
pub trait FnAbiGccExt < 'gcc , 'tcx > { fn gcc_type (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> FnAbiGcc < 'gcc > ; fn ptr_to_gcc_type (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; # [cfg (feature = "master")] fn gcc_cconv (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Option < FnAttribute < 'gcc > > ; }
};
}
