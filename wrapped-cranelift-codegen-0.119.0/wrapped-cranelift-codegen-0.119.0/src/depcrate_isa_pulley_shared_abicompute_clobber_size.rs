// Generated macro for compute_clobber_size (function)
macro_rules! Depcrate_isa_pulley_shared_abicompute_clobber_size {
() => {
// Module: crate::isa::pulley_shared::abi
// Provides: {"compute_clobber_size"}
// Dependencies: {}
fn compute_clobber_size (clobbers : & [Writable < RealReg >]) -> u32 { let mut clobbered_size = 0 ; for reg in clobbers { match reg . to_reg () . class () { RegClass :: Int => { clobbered_size += 8 ; } RegClass :: Float => { clobbered_size += 8 ; } RegClass :: Vector => unimplemented ! ("Vector Size Clobbered") , } } align_to (clobbered_size , 16) }
};
}
