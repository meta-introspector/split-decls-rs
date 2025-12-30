// Generated macro for produce_too_many_items (function)
macro_rules! Depcrate_iter_collect_testproduce_too_many_items {
() => {
// Module: crate::iter::collect::test
// Provides: {"produce_too_many_items"}
// Dependencies: {}
# [doc = " Promises to produce 2 items, but then produces 3.  Does not do any"] # [doc = " splits at all."] # [test] # [should_panic (expected = "too many values")] fn produce_too_many_items () { let mut v = vec ! [] ; collect_with_consumer (& mut v , 2 , | consumer | { let mut folder = consumer . into_folder () ; folder = folder . consume (22) ; folder = folder . consume (23) ; folder = folder . consume (24) ; unreachable ! ("folder does not complete") }) ; }
};
}
