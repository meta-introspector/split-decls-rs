// Generated macro for impl_467 (impl)
macro_rules! Depcrate_intrinsicimpl_467 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_467"}
// Dependencies: {}
impl < 'a , 'gcc , 'tcx > ArgAbiBuilderMethods < 'tcx > for Builder < 'a , 'gcc , 'tcx > { fn store_fn_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , idx : & mut usize , dst : PlaceRef < 'tcx , Self :: Value > ,) { arg_abi . store_fn_arg (self , idx , dst) } fn store_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , val : RValue < 'gcc > , dst : PlaceRef < 'tcx , RValue < 'gcc > > ,) { arg_abi . store (self , val , dst) } }
};
}
