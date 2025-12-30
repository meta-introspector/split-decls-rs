// Generated macro for LegacyPrefixes (enum)
macro_rules! Depcrate_isa_x64_encoding_rexLegacyPrefixes {
() => {
// Module: crate::isa::x64::encoding::rex
// Provides: {"LegacyPrefixes"}
// Dependencies: {}
# [doc = " We may need to include one or more legacy prefix bytes before the REX prefix.  This enum"] # [doc = " covers only the small set of possibilities that we actually need."] # [derive (PartialEq)] pub enum LegacyPrefixes { # [doc = " No prefix bytes."] None , # [doc = " Operand Size Override -- here, denoting \"16-bit operation\"."] _66 , # [doc = " The Lock prefix."] _F0 , # [doc = " Operand size override and Lock."] _66F0 , # [doc = " REPNE, but no specific meaning here -- is just an opcode extension."] _F2 , # [doc = " REP/REPE, but no specific meaning here -- is just an opcode extension."] _F3 , # [doc = " Operand size override and same effect as F3."] _66F3 , }
};
}
