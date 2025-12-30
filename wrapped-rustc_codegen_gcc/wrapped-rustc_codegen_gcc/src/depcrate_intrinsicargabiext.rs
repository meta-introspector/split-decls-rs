// Generated macro for ArgAbiExt (trait)
macro_rules! Depcrate_intrinsicArgAbiExt {
() => {
// Module: crate::intrinsic
// Provides: {"ArgAbiExt"}
// Dependencies: {}
pub trait ArgAbiExt < 'gcc , 'tcx > { fn store (& self , bx : & mut Builder < '_ , 'gcc , 'tcx > , val : RValue < 'gcc > , dst : PlaceRef < 'tcx , RValue < 'gcc > > ,) ; fn store_fn_arg (& self , bx : & mut Builder < '_ , 'gcc , 'tcx > , idx : & mut usize , dst : PlaceRef < 'tcx , RValue < 'gcc > > ,) ; }
};
}
