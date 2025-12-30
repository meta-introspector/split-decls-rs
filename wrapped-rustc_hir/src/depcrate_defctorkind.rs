// Generated macro for CtorKind (enum)
macro_rules! Depcrate_defCtorKind {
() => {
// Module: crate::def
// Provides: {"CtorKind"}
// Dependencies: {}
# [doc = " What kind of constructor something is."] # [derive (Clone , Copy , PartialEq , Eq , Encodable , Decodable , Hash , Debug , HashStable_Generic)] pub enum CtorKind { # [doc = " Constructor function automatically created by a tuple struct/variant."] Fn , # [doc = " Constructor constant automatically created by a unit struct/variant."] Const , }
};
}
