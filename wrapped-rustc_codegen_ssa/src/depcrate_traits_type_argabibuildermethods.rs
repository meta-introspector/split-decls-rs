// Generated macro for ArgAbiBuilderMethods (trait)
macro_rules! Depcrate_traits_type_ArgAbiBuilderMethods {
() => {
// Module: crate::traits::type_
// Provides: {"ArgAbiBuilderMethods"}
// Dependencies: {}
pub trait ArgAbiBuilderMethods < 'tcx > : BackendTypes { fn store_fn_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , idx : & mut usize , dst : PlaceRef < 'tcx , Self :: Value > ,) ; fn store_arg (& mut self , arg_abi : & ArgAbi < 'tcx , Ty < 'tcx > > , val : Self :: Value , dst : PlaceRef < 'tcx , Self :: Value > ,) ; }
};
}
