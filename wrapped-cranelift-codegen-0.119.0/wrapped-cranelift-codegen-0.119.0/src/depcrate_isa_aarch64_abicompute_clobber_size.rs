// Generated macro for compute_clobber_size (function)
macro_rules! Depcrate_isa_aarch64_abicompute_clobber_size {
() => {
// Module: crate::isa::aarch64::abi
// Provides: {"compute_clobber_size"}
// Dependencies: {}
fn compute_clobber_size (clobbered_callee_saves : & [Writable < RealReg >]) -> u32 { let mut int_regs = 0 ; let mut vec_regs = 0 ; for & reg in clobbered_callee_saves { match reg . to_reg () . class () { RegClass :: Int => { int_regs += 1 ; } RegClass :: Float => { vec_regs += 1 ; } RegClass :: Vector => unreachable ! () , } } let int_save_bytes = (int_regs + (int_regs & 1)) * 8 ; let vec_reg_size = 8 ; let vec_save_padding = vec_regs & 1 ; let vec_save_bytes = (vec_regs + vec_save_padding) * vec_reg_size ; int_save_bytes + vec_save_bytes }
};
}
