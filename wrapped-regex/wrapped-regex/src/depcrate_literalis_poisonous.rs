// Generated macro for is_poisonous (function)
macro_rules! Depcrate_literalis_poisonous {
() => {
// Module: crate::literal
// Provides: {"is_poisonous"}
// Dependencies: {}
# [doc = " Returns true if it is believe that this literal is likely to match very"] # [doc = " frequently, and is thus not a good candidate for a prefilter."] fn is_poisonous (lit : & Literal) -> bool { use regex_syntax :: hir :: literal :: rank ; lit . is_empty () || (lit . len () == 1 && rank (lit . as_bytes () [0]) >= 250) }
};
}
