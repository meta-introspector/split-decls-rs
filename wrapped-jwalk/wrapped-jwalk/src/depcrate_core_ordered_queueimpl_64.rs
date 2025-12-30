// Generated macro for impl_64 (impl)
macro_rules! Depcrate_core_ordered_queueimpl_64 {
() => {
// Module: crate::core::ordered_queue
// Provides: {"impl_64"}
// Dependencies: {}
impl OrderedMatcher { fn is_none (& self) -> bool { self . looking_for . is_empty () } fn decrement_remaining_children (& mut self) { * self . child_count_stack . last_mut () . unwrap () -= 1 ; } fn advance_past < T > (& mut self , ordered : & Ordered < T >) { self . decrement_remaining_children () ; if ordered . child_count > 0 { self . looking_for . push (0) ; self . child_count_stack . push (ordered . child_count) ; } else { self . looking_for . increment_last () ; while ! self . child_count_stack . is_empty () && * self . child_count_stack . last () . unwrap () == 0 { self . looking_for . pop () ; self . child_count_stack . pop () ; if ! self . looking_for . is_empty () { self . looking_for . increment_last () ; } } } } }
};
}
