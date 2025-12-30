// Generated macro for impl_63 (impl)
macro_rules! Depcrate_search_outcomeimpl_63 {
() => {
// Module: crate::search::outcome
// Provides: {"impl_63"}
// Dependencies: {}
impl Outcome { # [doc = " Given a list of `attrs` by order, return true if at least one of them is not set"] pub (crate) fn has_unspecified_attributes (& self , mut attrs : impl Iterator < Item = AttributeId >) -> bool { attrs . any (| order | self . matches_by_id [order . 0] . r#match . is_none ()) } # [doc = " Return the amount of attributes haven't yet been found."] # [doc = ""] # [doc = " If this number reaches 0, then the search can be stopped as there is nothing more to fill in."] pub (crate) fn remaining (& self) -> usize { self . remaining . expect ("BUG: instance must be initialized for each search set") } fn reduce_and_check_if_done (& mut self , attr : AttributeId) -> bool { if self . selected . is_empty () || self . selected . iter () . any (| (_name , id) | * id == Some (attr)) { * self . remaining . as_mut () . expect ("initialized") -= 1 ; } self . is_done () } }
};
}
