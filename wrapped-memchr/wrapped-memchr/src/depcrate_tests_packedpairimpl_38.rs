// Generated macro for impl_38 (impl)
macro_rules! Depcrate_tests_packedpairimpl_38 {
() => {
// Module: crate::tests::packedpair
// Provides: {"impl_38"}
// Dependencies: {}
impl Runner { # [doc = " Create a new test runner for \"packed pair\" substring search."] pub (crate) fn new () -> Runner { Runner { fwd : None } } # [doc = " Run all tests. This panics on the first failure."] # [doc = ""] # [doc = " If the implementation being tested returns `None` for a particular"] # [doc = " haystack/needle combination, then that test is skipped."] # [doc = ""] # [doc = " This runs tests on both the forward and reverse implementations given."] # [doc = " If either (or both) are missing, then tests for that implementation are"] # [doc = " skipped."] pub (crate) fn run (self) { if let Some (mut fwd) = self . fwd { for seed in SEEDS . iter () { for t in seed . generate () { match fwd (& t . haystack , & t . needle , t . index1 , t . index2) { None => continue , Some (result) => { assert_eq ! (t . fwd , result , "FORWARD, needle: {:?}, haystack: {:?}, \
                                 index1: {:?}, index2: {:?}" , t . needle , t . haystack , t . index1 , t . index2 ,) } } } } } } # [doc = " Set the implementation for forward \"packed pair\" substring search."] # [doc = ""] # [doc = " If the closure returns `None`, then it is assumed that the given"] # [doc = " test cannot be applied to the particular implementation and it is"] # [doc = " skipped. For example, if a particular implementation only supports"] # [doc = " needles or haystacks for some minimum length."] # [doc = ""] # [doc = " If this is not set, then forward \"packed pair\" search is not tested."] pub (crate) fn fwd (mut self , search : impl FnMut (& [u8] , & [u8] , u8 , u8) -> Option < Option < usize > > + 'static ,) -> Runner { self . fwd = Some (Box :: new (search)) ; self } }
};
}
