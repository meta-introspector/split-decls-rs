// Generated macro for Num (trait)
macro_rules! Depcrate_parseNum {
() => {
// Module: crate::parse
// Provides: {"Num"}
// Dependencies: {}
pub trait Num { fn from_u8 (x : u8) -> Self ; # [doc = " Returns `true` on overflow"] fn checked_mul_ext (& mut self , x : u8) -> bool ; # [doc = " Returns `true` on overflow"] fn checked_add_ext (& mut self , x : u8) -> bool ; # [doc = " Returns `true` on overflow"] fn checked_sub_ext (& mut self , x : u8) -> bool ; }
};
}
