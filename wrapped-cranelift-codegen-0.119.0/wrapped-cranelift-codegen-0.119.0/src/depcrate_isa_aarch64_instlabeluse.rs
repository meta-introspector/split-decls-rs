// Generated macro for LabelUse (enum)
macro_rules! Depcrate_isa_aarch64_instLabelUse {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"LabelUse"}
// Dependencies: {}
# [doc = " Different forms of label references for different instruction formats."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum LabelUse { # [doc = " 14-bit branch offset (conditional branches). PC-rel, offset is imm <<"] # [doc = " 2. Immediate is 14 signed bits, in bits 18:5. Used by tbz and tbnz."] Branch14 , # [doc = " 19-bit branch offset (conditional branches). PC-rel, offset is imm << 2. Immediate is 19"] # [doc = " signed bits, in bits 23:5. Used by cbz, cbnz, b.cond."] Branch19 , # [doc = " 26-bit branch offset (unconditional branches). PC-rel, offset is imm << 2. Immediate is 26"] # [doc = " signed bits, in bits 25:0. Used by b, bl."] Branch26 , # [allow (dead_code)] # [doc = " 19-bit offset for LDR (load literal). PC-rel, offset is imm << 2. Immediate is 19 signed bits,"] # [doc = " in bits 23:5."] Ldr19 , # [allow (dead_code)] # [doc = " 21-bit offset for ADR (get address of label). PC-rel, offset is not shifted. Immediate is"] # [doc = " 21 signed bits, with high 19 bits in bits 23:5 and low 2 bits in bits 30:29."] Adr21 , # [doc = " 32-bit PC relative constant offset (from address of constant itself),"] # [doc = " signed. Used in jump tables."] PCRel32 , }
};
}
