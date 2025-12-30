// Generated macro for impl_191 (impl)
macro_rules! Depcrate_rt_pathimpl_191 {
() => {
// Module: crate::rt::path
// Provides: {"impl_191"}
// Dependencies: {}
impl Schedule { # [doc = " Returns the index of the currently active thread"] fn active_thread_index (& self) -> Option < u8 > { self . threads . iter () . enumerate () . find (| (_ , th) | th . is_active ()) . map (| (index , _) | index as u8) } # [doc = " Compute the number of preemptions for the current state of the branch"] fn preemptions (& self) -> u8 { if self . initial_active . is_some () && self . initial_active != self . active_thread_index () { return self . preemptions + 1 ; } self . preemptions } fn backtrack (& mut self , thread_id : thread :: Id , preemption_bound : Option < u8 >) { assert ! (self . exploring) ; if let Some (bound) = preemption_bound { assert ! (self . preemptions <= bound , "[loom internal bug] actual = {}, bound = {}" , self . preemptions , bound) ; if self . preemptions == bound { return ; } } let thread_id = thread_id . as_usize () ; if thread_id >= self . threads . len () { return ; } if self . threads [thread_id] . is_enabled () { self . threads [thread_id] . explore () ; } else { for th in & mut self . threads { th . explore () ; } } } }
};
}
