// Generated macro for ShimSig (struct)
macro_rules! Depcrate_shims_sigShimSig {
() => {
// Module: crate::shims::sig
// Provides: {"ShimSig"}
// Dependencies: {}
# [doc = " Describes the expected signature of a shim."] pub struct ShimSig < 'tcx , const ARGS : usize > { pub abi : ExternAbi , pub args : [Ty < 'tcx > ; ARGS] , pub ret : Ty < 'tcx > , }
};
}
