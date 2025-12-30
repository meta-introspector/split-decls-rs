// Generated macro for compute_clobber_size (function)
macro_rules! Depcrate_isa_x64_abicompute_clobber_size {
() => {
// Module: crate::isa::x64::abi
// Provides: {"compute_clobber_size"}
// Dependencies: {}
fn compute_clobber_size (clobbers : & [Writable < RealReg >]) -> u32 { let mut clobbered_size = 0 ; for reg in clobbers { match reg . to_reg () . class () { RegClass :: Int => { clobbered_size += 8 ; } RegClass :: Float => { clobbered_size = align_to (clobbered_size , 16) ; clobbered_size += 16 ; } RegClass :: Vector => unreachable ! () , } } align_to (clobbered_size , 16) }
};
}
