// Generated macro for int_type (function)
macro_rules! Depcrate_interfaceint_type {
() => {
// Module: crate::interface
// Provides: {"int_type"}
// Dependencies: {}
fn int_type (int : Int) -> & 'static str { match int { Int :: U8 => "byte" , Int :: U16 => "ushort" , Int :: U32 => "uint" , Int :: U64 => "ulong" , } }
};
}
