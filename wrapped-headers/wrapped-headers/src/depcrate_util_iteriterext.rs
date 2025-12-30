// Generated macro for IterExt (trait)
macro_rules! Depcrate_util_iterIterExt {
() => {
// Module: crate::util::iter
// Provides: {"IterExt"}
// Dependencies: {}
pub trait IterExt : Iterator { fn just_one (& mut self) -> Option < Self :: Item > { let one = self . next () ? ; match self . next () { Some (_) => None , None => Some (one) , } } }
};
}
