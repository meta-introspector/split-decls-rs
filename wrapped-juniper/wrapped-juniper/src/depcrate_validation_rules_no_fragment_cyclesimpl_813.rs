// Generated macro for impl_813 (impl)
macro_rules! Depcrate_validation_rules_no_fragment_cyclesimpl_813 {
() => {
// Module: crate::validation::rules::no_fragment_cycles
// Provides: {"impl_813"}
// Dependencies: {}
impl < 'a > CycleDetector < 'a > { fn detect_from (& mut self , from : & 'a str) { let mut to_visit = Vec :: new () ; to_visit . push ((from , Vec :: new () , HashMap :: new ())) ; while let Some ((from , path , path_indices)) = to_visit . pop () { to_visit . extend (self . detect_from_inner (from , path , path_indices)) ; } } # [doc = " This function should be called only inside [`Self::detect_from()`], as"] # [doc = " it's a recursive function using heap instead of a stack. So, instead of"] # [doc = " the recursive call, we return a [`Vec`] that is visited inside"] # [doc = " [`Self::detect_from()`]."] fn detect_from_inner (& mut self , from : & 'a str , path : Vec < & 'a BorrowedSpanning < 'a , str > > , mut path_indices : HashMap < & 'a str , usize > ,) -> Vec < CycleDetectorState < 'a > > { self . visited . insert (from) ; if ! self . spreads . contains_key (from) { return Vec :: new () ; } path_indices . insert (from , path . len ()) ; let mut to_visit = Vec :: new () ; for node in & self . spreads [from] { let name = node . item ; let index = path_indices . get (name) . cloned () ; if let Some (index) = index { let err_pos = if index < path . len () { path [index] } else { node } ; self . errors . push (RuleError :: new (& error_message (name) , & [err_pos . span . start])) ; } else { let mut path = path . clone () ; path . push (node) ; to_visit . push ((name , path , path_indices . clone ())) ; } } to_visit } }
};
}
