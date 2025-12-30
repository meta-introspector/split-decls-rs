// Generated macro for produce_fewer_items (function)
macro_rules! Depcrate_iter_collect_testproduce_fewer_items {
() => {
// Module: crate::iter::collect::test
// Provides: {"produce_fewer_items"}
// Dependencies: {}
# [doc = " Produces fewer items than promised. Does not do any"] # [doc = " splits at all."] # [test] # [should_panic (expected = "expected 5 total writes, but got 2")] fn produce_fewer_items () { let mut v = vec ! [] ; collect_with_consumer (& mut v , 5 , | consumer | { let mut folder = consumer . into_folder () ; folder = folder . consume (22) ; folder = folder . consume (23) ; folder . complete () }) ; }
};
}
