// Generated macro for impl_201 (impl)
macro_rules! Depcrate_dirwalk_iterimpl_201 {
() => {
// Module: crate::dirwalk::iter
// Provides: {"impl_201"}
// Dependencies: {}
# [doc = " Access"] impl Iter { # [doc = " Return the outcome of the iteration, or `None` if the iterator isn't fully consumed."] pub fn outcome_mut (& mut self) -> Option < & mut Outcome > { self . out . as_mut () } # [doc = " Turn the iterator into the iteration outcome, which is `None` on error or if the iteration"] # [doc = " isn't complete."] pub fn into_outcome (mut self) -> Option < Outcome > { self . out . take () } }
};
}
