// Generated macro for FieldlessVariantsStrategy (enum)
macro_rules! Depcrate_deriving_genericFieldlessVariantsStrategy {
() => {
// Module: crate::deriving::generic
// Provides: {"FieldlessVariantsStrategy"}
// Dependencies: {}
# [doc = " How to handle fieldless enum variants."] # [derive (PartialEq)] pub (crate) enum FieldlessVariantsStrategy { # [doc = " Combine fieldless variants into a single match arm."] # [doc = " This assumes that relevant information has been handled"] # [doc = " by looking at the enum's discriminant."] Unify , # [doc = " Don't do anything special about fieldless variants. They are"] # [doc = " handled like any other variant."] Default , # [doc = " If all variants of the enum are fieldless, expand the special"] # [doc = " `AllFieldLessEnum` substructure, so that the entire enum can be handled"] # [doc = " at once."] SpecializeIfAllVariantsFieldless , }
};
}
