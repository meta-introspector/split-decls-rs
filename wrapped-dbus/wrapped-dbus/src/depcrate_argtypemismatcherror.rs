// Generated macro for TypeMismatchError (struct)
macro_rules! Depcrate_argTypeMismatchError {
() => {
// Module: crate::arg
// Provides: {"TypeMismatchError"}
// Dependencies: {}
# [doc = " Error struct to indicate a D-Bus argument type mismatch."] # [doc = ""] # [doc = " Might be returned from `iter::read()`."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub struct TypeMismatchError { expected : ArgType , found : ArgType , position : u32 , }
};
}
