// Generated macro for checked_add (function)
macro_rules! Depcrate_instructionchecked_add {
() => {
// Module: crate::instruction
// Provides: {"checked_add"}
// Dependencies: {}
# [doc = " Addition that returns [`InstructionError::InsufficientFunds`] on overflow."] # [doc = ""] # [doc = " This is an internal utility function."] # [doc (hidden)] pub fn checked_add (a : u64 , b : u64) -> Result < u64 , InstructionError > { a . checked_add (b) . ok_or (InstructionError :: InsufficientFunds) }
};
}
