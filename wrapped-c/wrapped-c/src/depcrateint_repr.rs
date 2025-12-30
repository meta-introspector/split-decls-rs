// Generated macro for int_repr (function)
macro_rules! Depcrateint_repr {
() => {
// Module: crate
// Provides: {"int_repr"}
// Dependencies: {}
pub fn int_repr (ty : Int) -> & 'static str { match ty { Int :: U8 => "uint8_t" , Int :: U16 => "uint16_t" , Int :: U32 => "uint32_t" , Int :: U64 => "uint64_t" , } }
};
}
