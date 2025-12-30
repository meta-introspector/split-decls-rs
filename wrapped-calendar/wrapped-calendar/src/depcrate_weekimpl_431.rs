// Generated macro for impl_431 (impl)
macro_rules! Depcrate_weekimpl_431 {
() => {
// Module: crate::week
// Provides: {"impl_431"}
// Dependencies: {}
impl Iterator for WeekdaySetIterator { type Item = Weekday ; fn next (& mut self) -> Option < Self :: Item > { while self . current_day . next_day () != self . first_weekday { if self . weekend . contains (self . current_day) { let result = self . current_day ; self . current_day = self . current_day . next_day () ; return Some (result) ; } else { self . current_day = self . current_day . next_day () ; } } if self . weekend . contains (self . current_day) { self . weekend = WeekdaySet :: new (& []) ; return Some (self . current_day) ; } Option :: None } }
};
}
