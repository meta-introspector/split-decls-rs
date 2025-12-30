// Generated macro for count_elements (function)
macro_rules! Depcrate_matcher_support_count_elementscount_elements {
() => {
// Module: crate::matcher_support::count_elements
// Provides: {"count_elements"}
// Dependencies: {}
# [doc = " Counts the number of elements in `value`."] # [doc = ""] # [doc = " This uses [`Iterator::size_hint`] when that function returns an"] # [doc = " unambiguous answer, i.e., the upper bound exists and the lower and upper"] # [doc = " bounds agree. Otherwise it iterates through `value` and counts the"] # [doc = " elements."] pub (crate) fn count_elements < ContainerT : IntoIterator > (value : ContainerT) -> usize { let iterator = value . into_iter () ; if let (lower , Some (higher)) = iterator . size_hint () { if lower == higher { return lower ; } } iterator . count () }
};
}
