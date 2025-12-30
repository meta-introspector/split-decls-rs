// Generated macro for impl_2060 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2060 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2060"}
// Dependencies: {}
# [doc = " float rounding mode."] impl FRM { pub (crate) fn to_static_str (self) -> & 'static str { match self { FRM :: RNE => "rne" , FRM :: RTZ => "rtz" , FRM :: RDN => "rdn" , FRM :: RUP => "rup" , FRM :: RMM => "rmm" , FRM :: Fcsr => "fcsr" , } } # [inline] pub (crate) fn bits (self) -> u8 { match self { FRM :: RNE => 0b000 , FRM :: RTZ => 0b001 , FRM :: RDN => 0b010 , FRM :: RUP => 0b011 , FRM :: RMM => 0b100 , FRM :: Fcsr => 0b111 , } } pub (crate) fn as_u32 (self) -> u32 { self . bits () as u32 } }
};
}
