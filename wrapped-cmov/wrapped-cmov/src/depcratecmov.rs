// Generated macro for Cmov (trait)
macro_rules! DepcrateCmov {
() => {
// Module: crate
// Provides: {"Cmov"}
// Dependencies: {}
# [doc = " Conditional move"] pub trait Cmov { # [doc = " Move if non-zero."] # [doc = ""] # [doc = " Uses a `test` instruction to check if the given `condition` value is"] # [doc = " equal to zero, conditionally moves `value` to `self` when `condition` is"] # [doc = " not equal to zero."] fn cmovnz (& mut self , value : & Self , condition : Condition) ; # [doc = " Move if zero."] # [doc = ""] # [doc = " Uses a `cmp` instruction to check if the given `condition` value is"] # [doc = " equal to zero, and if so, conditionally moves `value` to `self`"] # [doc = " when `condition` is equal to zero."] fn cmovz (& mut self , value : & Self , condition : Condition) { self . cmovnz (value , ! condition) } }
};
}
