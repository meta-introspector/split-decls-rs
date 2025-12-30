// Generated macro for TargetRef (enum)
macro_rules! DepcrateTargetRef {
() => {
// Module: crate
// Provides: {"TargetRef"}
// Dependencies: {}
# [doc = " Denotes a ref target, equivalent to [`Kind`], but with immutable data."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub enum TargetRef < 'a > { # [doc = " A ref that points directly to an object id."] Object (& 'a oid) , # [doc = " A ref that points to another reference by its validated name, adding a level of indirection."] Symbolic (& 'a FullNameRef) , }
};
}
