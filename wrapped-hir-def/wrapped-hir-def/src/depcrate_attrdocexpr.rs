// Generated macro for DocExpr (enum)
macro_rules! Depcrate_attrDocExpr {
() => {
// Module: crate::attr
// Provides: {"DocExpr"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum DocExpr { Invalid , # [doc = " eg. `#[doc(hidden)]`, `#[doc(alias = \"x\")]`"] Atom (DocAtom) , # [doc = " eg. `#[doc(alias(\"x\", \"y\"))]`"] Alias (Vec < Symbol >) , }
};
}
