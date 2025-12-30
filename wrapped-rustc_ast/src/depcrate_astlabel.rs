// Generated macro for Label (struct)
macro_rules! Depcrate_astLabel {
() => {
// Module: crate::ast
// Provides: {"Label"}
// Dependencies: {}
# [doc = " A \"Label\" is an identifier of some point in sources,"] # [doc = " e.g. in the following code:"] # [doc = ""] # [doc = " ```rust"] # [doc = " 'outer: loop {"] # [doc = "     break 'outer;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " `'outer` is a label."] # [derive (Clone , Encodable , Decodable , Copy , HashStable_Generic , Eq , PartialEq , Walkable)] pub struct Label { pub ident : Ident , }
};
}
