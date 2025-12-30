// Generated macro for check_disjoint_and_sort (function)
macro_rules! Depcrate_text_editcheck_disjoint_and_sort {
() => {
// Module: crate::text_edit
// Provides: {"check_disjoint_and_sort"}
// Dependencies: {}
fn check_disjoint_and_sort (indels : & mut [Indel]) -> bool { indels . sort_by_key (| indel | (indel . delete . start () , indel . delete . end ())) ; check_disjoint (& mut indels . iter ()) }
};
}
