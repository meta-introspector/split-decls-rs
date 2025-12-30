// Generated macro for AttributeSafety (enum)
macro_rules! Depcrate_builtin_attrsAttributeSafety {
() => {
// Module: crate::builtin_attrs
// Provides: {"AttributeSafety"}
// Dependencies: {}
# [derive (Copy , Clone , PartialEq , Debug)] pub enum AttributeSafety { # [doc = " Normal attribute that does not need `#[unsafe(...)]`"] Normal , # [doc = " Unsafe attribute that requires safety obligations to be discharged."] # [doc = ""] # [doc = " An error is emitted when `#[unsafe(...)]` is omitted, except when the attribute's edition"] # [doc = " is less than the one stored in `unsafe_since`. This handles attributes that were safe in"] # [doc = " earlier editions, but become unsafe in later ones."] Unsafe { unsafe_since : Option < Edition > } , }
};
}
