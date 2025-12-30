// Generated macro for ObjectKind (enum)
macro_rules! Depcrate_readObjectKind {
() => {
// Module: crate::read
// Provides: {"ObjectKind"}
// Dependencies: {}
# [doc = " An object kind."] # [doc = ""] # [doc = " Returned by [`Object::kind`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum ObjectKind { # [doc = " The object kind is unknown."] Unknown , # [doc = " Relocatable object."] Relocatable , # [doc = " Executable."] Executable , # [doc = " Dynamic shared object."] Dynamic , # [doc = " Core."] Core , }
};
}
