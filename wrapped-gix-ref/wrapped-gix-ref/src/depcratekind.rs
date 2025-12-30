// Generated macro for Kind (enum)
macro_rules! DepcrateKind {
() => {
// Module: crate
// Provides: {"Kind"}
// Dependencies: {}
# [doc = " Denotes the kind of reference."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Kind { # [doc = " A ref that points to an object id directly."] Object , # [doc = " A ref that points to another reference, adding a level of indirection."] # [doc = ""] # [doc = " It can be resolved to an id using the [`peel_to_id()`][`crate::file::ReferenceExt::peel_to_id()`] method."] Symbolic , }
};
}
