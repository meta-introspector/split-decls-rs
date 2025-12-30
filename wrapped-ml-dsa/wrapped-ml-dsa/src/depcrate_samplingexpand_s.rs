// Generated macro for expand_s (function)
macro_rules! Depcrate_samplingexpand_s {
() => {
// Module: crate::sampling
// Provides: {"expand_s"}
// Dependencies: {}
pub (crate) fn expand_s < K : ArraySize > (rho : & [u8] , eta : Eta , base : usize) -> Vector < K > { Vector :: new (Array :: from_fn (| r | { let r = Truncate :: truncate (r + base) ; rej_bounded_poly (rho , eta , r) })) }
};
}
