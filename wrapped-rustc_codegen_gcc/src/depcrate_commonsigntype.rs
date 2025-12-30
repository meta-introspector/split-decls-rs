// Generated macro for SignType (trait)
macro_rules! Depcrate_commonSignType {
() => {
// Module: crate::common
// Provides: {"SignType"}
// Dependencies: {}
pub trait SignType < 'gcc , 'tcx > { fn is_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn is_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> bool ; fn to_signed (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; fn to_unsigned (& self , cx : & CodegenCx < 'gcc , 'tcx >) -> Type < 'gcc > ; }
};
}
