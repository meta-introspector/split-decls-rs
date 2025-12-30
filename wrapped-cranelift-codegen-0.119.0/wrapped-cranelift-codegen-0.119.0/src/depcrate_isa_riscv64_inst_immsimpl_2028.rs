// Generated macro for impl_2028 (impl)
macro_rules! Depcrate_isa_riscv64_inst_immsimpl_2028 {
() => {
// Module: crate::isa::riscv64::inst::imms
// Provides: {"impl_2028"}
// Dependencies: {}
impl Inst { pub (crate) fn imm_min () -> i64 { let imm20_max : i64 = (1 << 19) << 12 ; let imm12_max = 1 << 11 ; - imm20_max - imm12_max } pub (crate) fn imm_max () -> i64 { let imm20_max : i64 = ((1 << 19) - 1) << 12 ; let imm12_max = (1 << 11) - 1 ; imm20_max + imm12_max } # [doc = " An imm20 immediate and an Imm12 immediate can generate a 32-bit immediate."] # [doc = " This helper produces an imm12, imm20, or both to generate the value."] # [doc = ""] # [doc = " `value` must be between `imm_min()` and `imm_max()`, or else"] # [doc = " this helper returns `None`."] pub (crate) fn generate_imm (value : u64) -> Option < (Imm20 , Imm12) > { if let Some (imm12) = Imm12 :: maybe_from_u64 (value) { return Some ((Imm20 :: ZERO , imm12)) ; } let value = value as i64 ; if ! (value >= Self :: imm_min () && value <= Self :: imm_max ()) { return None ; } const MOD_NUM : i64 = 4096 ; let (imm20 , imm12) = if value > 0 { let mut imm20 = value / MOD_NUM ; let mut imm12 = value % MOD_NUM ; if imm12 >= 2048 { imm12 -= MOD_NUM ; imm20 += 1 ; } assert ! (imm12 >= - 2048 && imm12 <= 2047) ; (imm20 , imm12) } else { let value_abs = value . abs () ; let imm20 = value_abs / MOD_NUM ; let imm12 = value_abs % MOD_NUM ; let mut imm20 = - imm20 ; let mut imm12 = - imm12 ; if imm12 < - 2048 { imm12 += MOD_NUM ; imm20 -= 1 ; } (imm20 , imm12) } ; assert ! (imm20 != 0 || imm12 != 0) ; let imm20 = i32 :: try_from (imm20) . unwrap () ; let imm12 = i16 :: try_from (imm12) . unwrap () ; Some ((Imm20 :: from_i32 (imm20) , Imm12 :: from_i16 (imm12))) } }
};
}
