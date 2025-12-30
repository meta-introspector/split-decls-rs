// Generated macro for impl_289 (impl)
macro_rules! Depcrate_validation_rules_no_fragment_cyclesimpl_289 {
() => {
// Module: crate::validation::rules::no_fragment_cycles
// Provides: {"impl_289"}
// Dependencies: {}
impl < 'a > CycleDetector < 'a > { fn detect_from (& mut self , from : & 'a str , path : & mut Vec < (& 'a str , Pos) >) { self . visited . insert (from) ; if ! self . spreads . contains_key (from) { return ; } self . path_indices . insert (from , path . len ()) ; for (name , pos) in & self . spreads [from] { let index = self . path_indices . get (name) . cloned () ; if let Some (index) = index { let err_pos = if index < path . len () { path [index] . 1 } else { * pos } ; self . errors . push (RuleError :: new (vec ! [err_pos] , format ! ("Cannot spread fragment \"{}\"" , name) ,)) ; } else if ! self . visited . contains (name) { path . push ((name , * pos)) ; self . detect_from (name , path) ; path . pop () ; } } self . path_indices . remove (from) ; } }
};
}
