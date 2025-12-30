// Generated macro for FromIntegerLiteral (trait)
macro_rules! Depcrate_integerFromIntegerLiteral {
() => {
// Module: crate::integer
// Provides: {"FromIntegerLiteral"}
// Dependencies: {}
# [doc = " Integer literal types. *Implementation detail*."] # [doc = ""] # [doc = " Implemented for all integer literal types. This trait is sealed and cannot"] # [doc = " be implemented outside of this crate. The trait's methods are implementation"] # [doc = " detail of this library and are not subject to semver."] pub trait FromIntegerLiteral : self :: sealed :: Sealed + Copy { # [doc = " Creates itself from the given number. `n` is guaranteed to be `<= 16`."] # [doc (hidden)] fn from_small_number (n : u8) -> Self ; # [doc (hidden)] fn checked_add (self , rhs : Self) -> Option < Self > ; # [doc (hidden)] fn checked_mul (self , rhs : Self) -> Option < Self > ; # [doc (hidden)] fn ty () -> IntegerType ; }
};
}
