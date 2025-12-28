macro_rules! R_LARCH_GOT_PC_HI20 {
    () => {
        # [doc = " The signed 32-bit offset `offs` from `PC & 0xfffff000` to"] # [doc = " `(GP + G + 0x800) & 0xfffff000`, with 12 trailing zeros removed."] # [doc = ""] # [doc = " We define the *PC relative anchor* for the GOT entry at `GP + G` as"] # [doc = " `PC + offs` (`offs` is sign-extended to VA bits)."] pub const R_LARCH_GOT_PC_HI20 : u32 = 75 ;
    };
}

R_LARCH_GOT_PC_HI20!()