// Generated macro for R_LARCH_PCALA_HI20 (const)
macro_rules! Depcrate_elfR_LARCH_PCALA_HI20 {
() => {
// Module: crate::elf
// Provides: {"R_LARCH_PCALA_HI20"}
// Dependencies: {}
# [doc = " The signed 32-bit offset `offs` from `PC & 0xfffff000` to"] # [doc = " `(S + A + 0x800) & 0xfffff000`, with 12 trailing zeros removed."] # [doc = ""] # [doc = " We define the *PC relative anchor* for `S + A` as `PC + offs` (`offs`"] # [doc = " is sign-extended to VA bits)."] pub const R_LARCH_PCALA_HI20 : u32 = 71 ;
};
}
