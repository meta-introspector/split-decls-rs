// Generated macro for test (module)
macro_rules! Depcrate_isa_riscv64_inst_immstest {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_imm12 () { let x = Imm12 :: ZERO ; assert_eq ! (0 , x . bits ()) ; Imm12 :: maybe_from_u64 (0xffff_ffff_ffff_ffff) . unwrap () ; } # [test] fn imm20_and_imm12 () { assert ! (Inst :: imm_max () == (i32 :: MAX - 2048) as i64) ; assert ! (Inst :: imm_min () == i32 :: MIN as i64 - 2048) ; } }
};
}
