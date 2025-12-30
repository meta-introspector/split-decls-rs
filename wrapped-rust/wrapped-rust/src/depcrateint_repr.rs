// Generated macro for int_repr (function)
macro_rules! Depcrateint_repr {
() => {
// Module: crate
// Provides: {"int_repr"}
// Dependencies: {}
fn int_repr (repr : Int) -> & 'static str { match repr { Int :: U8 => "u8" , Int :: U16 => "u16" , Int :: U32 => "u32" , Int :: U64 => "u64" , } }
};
}
