// Generated macro for find_insert_pos_by_order (function)
macro_rules! Depcrate_file_utilfind_insert_pos_by_order {
() => {
// Module: crate::file::util
// Provides: {"find_insert_pos_by_order"}
// Dependencies: {}
fn find_insert_pos_by_order (sections_with_name : & [SectionId] , before_order : usize , lookup_section_order : impl Fn (SectionId) -> usize ,) -> usize { let mut insert_pos = sections_with_name . len () ; for (idx , candidate_id) in sections_with_name . iter () . enumerate () { let candidate_order = lookup_section_order (* candidate_id) ; match candidate_order . cmp (& before_order) { Ordering :: Less => { } Ordering :: Equal => { insert_pos = idx + 1 ; break ; } Ordering :: Greater => { insert_pos = idx ; break ; } } } insert_pos }
};
}
