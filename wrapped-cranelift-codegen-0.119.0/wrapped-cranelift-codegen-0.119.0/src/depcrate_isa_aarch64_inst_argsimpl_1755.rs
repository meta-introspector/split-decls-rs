// Generated macro for impl_1755 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1755 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1755"}
// Dependencies: {}
impl APIKey { # [doc = " Returns the encoding of the `auti{key}` instruction used to decrypt the"] # [doc = " `lr` register."] pub fn enc_auti_hint (& self) -> u32 { let (crm , op2) = match self { APIKey :: AZ => (0b0011 , 0b100) , APIKey :: ASP => (0b0011 , 0b101) , APIKey :: BZ => (0b0011 , 0b110) , APIKey :: BSP => (0b0011 , 0b111) , } ; 0xd503201f | (crm << 8) | (op2 << 5) } }
};
}
