// Generated macro for TagMode (enum)
macro_rules! Depcrate_tagTagMode {
() => {
// Module: crate::tag
// Provides: {"TagMode"}
// Dependencies: {}
# [doc = " Tagging modes: `EXPLICIT` versus `IMPLICIT`."] # [derive (Copy , Clone , Debug , Default , Eq , PartialEq , PartialOrd , Ord)] pub (crate) enum TagMode { # [doc = " `EXPLICIT` tagging."] # [doc = ""] # [doc = " Tag is added in addition to the inner tag of the type."] # [default] Explicit , # [doc = " `IMPLICIT` tagging."] # [doc = ""] # [doc = " Tag replaces the existing tag of the inner type."] Implicit , }
};
}
