// Generated macro for tests (module)
macro_rules! Depcrate_matcher_support_count_elementstests {
() => {
// Module: crate::matcher_support::count_elements
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: prelude :: * ; use crate :: Result ; # [test] fn count_elements_vec () -> Result < () > { verify_that ! (count_elements (vec ! [1 , 2 , 3]) , eq (3)) } # [test] fn count_elements_with_imprecise_hint () -> Result < () > { struct FakeIterator ; impl Iterator for FakeIterator { type Item = () ; fn next (& mut self) -> Option < Self :: Item > { None } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (123)) } } verify_that ! (count_elements (FakeIterator) , eq (0)) } # [test] fn count_elements_with_no_hint () -> Result < () > { struct FakeIterator ; impl Iterator for FakeIterator { type Item = () ; fn next (& mut self) -> Option < Self :: Item > { None } fn size_hint (& self) -> (usize , Option < usize >) { (0 , None) } } verify_that ! (count_elements (FakeIterator) , eq (0)) } }
};
}
