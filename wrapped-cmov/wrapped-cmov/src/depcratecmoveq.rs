// Generated macro for CmovEq (trait)
macro_rules! DepcrateCmovEq {
() => {
// Module: crate
// Provides: {"CmovEq"}
// Dependencies: {}
# [doc = " Conditional move with equality comparison"] pub trait CmovEq { # [doc = " Move if both inputs are equal."] # [doc = ""] # [doc = " Uses a `xor` instruction to compare the two values, and"] # [doc = " conditionally moves `input` to `output` when they are equal."] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) ; # [doc = " Move if both inputs are not equal."] # [doc = ""] # [doc = " Uses a `xor` instruction to compare the two values, and"] # [doc = " conditionally moves `input` to `output` when they are not equal."] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { let mut tmp = 1u8 ; self . cmoveq (rhs , 0u8 , & mut tmp) ; tmp . cmoveq (& 1u8 , input , output) ; } }
};
}
