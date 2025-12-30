// Generated macro for impl_1353 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1353 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1353"}
// Dependencies: {}
impl fmt :: Debug for ShiftKind { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { let name = match self { ShiftKind :: ShiftLeft => "shl" , ShiftKind :: ShiftRightLogical => "shr" , ShiftKind :: ShiftRightArithmetic => "sar" , ShiftKind :: RotateLeft => "rol" , ShiftKind :: RotateRight => "ror" , } ; write ! (fmt , "{name}") } }
};
}
