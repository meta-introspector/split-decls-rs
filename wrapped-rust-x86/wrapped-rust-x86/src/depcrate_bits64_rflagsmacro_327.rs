// Generated macro for macro_327 (macro)
macro_rules! Depcrate_bits64_rflagsmacro_327 {
() => {
// Module: crate::bits64::rflags
// Provides: {"macro_327"}
// Dependencies: {}
bitflags ! { # [doc = " The RFLAGS register."] # [doc = " This is duplicated code from bits32 eflags.rs."] pub struct RFlags : u64 { # [doc = " ID Flag (ID)"] const FLAGS_ID = 1 << 21 ; # [doc = " Virtual Interrupt Pending (VIP)"] const FLAGS_VIP = 1 << 20 ; # [doc = " Virtual Interrupt Flag (VIF)"] const FLAGS_VIF = 1 << 19 ; # [doc = " Alignment Check (AC)"] const FLAGS_AC = 1 << 18 ; # [doc = " Virtual-8086 Mode (VM)"] const FLAGS_VM = 1 << 17 ; # [doc = " Resume Flag (RF)"] const FLAGS_RF = 1 << 16 ; # [doc = " Nested Task (NT)"] const FLAGS_NT = 1 << 14 ; # [doc = " I/O Privilege Level (IOPL) 0"] const FLAGS_IOPL0 = 0b00 << 12 ; # [doc = " I/O Privilege Level (IOPL) 1"] const FLAGS_IOPL1 = 0b01 << 12 ; # [doc = " I/O Privilege Level (IOPL) 2"] const FLAGS_IOPL2 = 0b10 << 12 ; # [doc = " I/O Privilege Level (IOPL) 3"] const FLAGS_IOPL3 = 0b11 << 12 ; # [doc = " Overflow Flag (OF)"] const FLAGS_OF = 1 << 11 ; # [doc = " Direction Flag (DF)"] const FLAGS_DF = 1 << 10 ; # [doc = " Interrupt Enable Flag (IF)"] const FLAGS_IF = 1 << 9 ; # [doc = " Trap Flag (TF)"] const FLAGS_TF = 1 << 8 ; # [doc = " Sign Flag (SF)"] const FLAGS_SF = 1 << 7 ; # [doc = " Zero Flag (ZF)"] const FLAGS_ZF = 1 << 6 ; # [doc = " Auxiliary Carry Flag (AF)"] const FLAGS_AF = 1 << 4 ; # [doc = " Parity Flag (PF)"] const FLAGS_PF = 1 << 2 ; # [doc = " Bit 1 is always 1."] const FLAGS_A1 = 1 << 1 ; # [doc = " Carry Flag (CF)"] const FLAGS_CF = 1 << 0 ; } }
};
}
