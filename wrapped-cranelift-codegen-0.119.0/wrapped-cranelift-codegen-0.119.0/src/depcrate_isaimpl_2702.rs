// Generated macro for impl_2702 (impl)
macro_rules! Depcrate_isaimpl_2702 {
() => {
// Module: crate::isa
// Provides: {"impl_2702"}
// Dependencies: {}
# [doc = " Methods implemented for free for target ISA!"] impl < 'a > dyn TargetIsa + 'a { # [doc = " Get the default calling convention of this target."] pub fn default_call_conv (& self) -> CallConv { CallConv :: triple_default (self . triple ()) } # [doc = " Get the endianness of this ISA."] pub fn endianness (& self) -> ir :: Endianness { match self . triple () . endianness () . unwrap () { target_lexicon :: Endianness :: Little => ir :: Endianness :: Little , target_lexicon :: Endianness :: Big => ir :: Endianness :: Big , } } # [doc = " Returns the minimum symbol alignment for this ISA."] pub fn symbol_alignment (& self) -> u64 { match self . triple () . architecture { Architecture :: S390x => 2 , _ => 1 , } } # [doc = " Get the pointer type of this ISA."] pub fn pointer_type (& self) -> ir :: Type { ir :: Type :: int (self . pointer_bits () as u16) . unwrap () } # [doc = " Get the width of pointers on this ISA."] pub (crate) fn pointer_width (& self) -> PointerWidth { self . triple () . pointer_width () . unwrap () } # [doc = " Get the width of pointers on this ISA, in units of bits."] pub fn pointer_bits (& self) -> u8 { self . pointer_width () . bits () } # [doc = " Get the width of pointers on this ISA, in units of bytes."] pub fn pointer_bytes (& self) -> u8 { self . pointer_width () . bytes () } # [doc = " Get the information needed by frontends producing Cranelift IR."] pub fn frontend_config (& self) -> TargetFrontendConfig { TargetFrontendConfig { default_call_conv : self . default_call_conv () , pointer_width : self . pointer_width () , page_size_align_log2 : self . page_size_align_log2 () , } } }
};
}
