// Generated macro for CvQualifiers (struct)
macro_rules! Depcrate_astCvQualifiers {
() => {
// Module: crate::ast
// Provides: {"CvQualifiers"}
// Dependencies: {}
# [doc = " The `<CV-qualifiers>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <CV-qualifiers> ::= [r] [V] [K]   # restrict (C99), volatile, const"] # [doc = " ```"] # [derive (Clone , Debug , Default , Hash , PartialEq , Eq)] pub struct CvQualifiers { # [doc = " Is this `restrict` qualified?"] pub restrict : bool , # [doc = " Is this `volatile` qualified?"] pub volatile : bool , # [doc = " Is this `const` qualified?"] pub const_ : bool , }
};
}
