// Generated macro for impl_1393 (impl)
macro_rules! Depcrateimpl_1393 {
() => {
// Module: crate
// Provides: {"impl_1393"}
// Dependencies: {}
impl InitializationRequiringAction { fn as_noun (self) -> & 'static str { match self { InitializationRequiringAction :: Borrow => "borrow" , InitializationRequiringAction :: MatchOn => "use" , InitializationRequiringAction :: Use => "use" , InitializationRequiringAction :: Assignment => "assign" , InitializationRequiringAction :: PartialAssignment => "assign to part" , } } fn as_verb_in_past_tense (self) -> & 'static str { match self { InitializationRequiringAction :: Borrow => "borrowed" , InitializationRequiringAction :: MatchOn => "matched on" , InitializationRequiringAction :: Use => "used" , InitializationRequiringAction :: Assignment => "assigned" , InitializationRequiringAction :: PartialAssignment => "partially assigned" , } } fn as_general_verb_in_past_tense (self) -> & 'static str { match self { InitializationRequiringAction :: Borrow | InitializationRequiringAction :: MatchOn | InitializationRequiringAction :: Use => "used" , InitializationRequiringAction :: Assignment => "assigned" , InitializationRequiringAction :: PartialAssignment => "partially assigned" , } } }
};
}
