// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl Cmov for u8 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { let mut tmp = * self as u16 ; tmp . cmovnz (& (* value as u16) , condition) ; * self = tmp as u8 ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { let mut tmp = * self as u16 ; tmp . cmovz (& (* value as u16) , condition) ; * self = tmp as u8 ; } }
};
}
