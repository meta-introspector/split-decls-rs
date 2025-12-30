// Generated macro for impl_14 (impl)
macro_rules! Depcrate_strategyimpl_14 {
() => {
// Module: crate::strategy
// Provides: {"impl_14"}
// Dependencies: {}
impl < State , Transition , StateStrategy , TransitionStrategy > Sequential < State , Transition , StateStrategy , TransitionStrategy > where State : 'static , Transition : 'static , StateStrategy : 'static , TransitionStrategy : 'static , { pub fn new (size : SizeRange , init_state : impl Fn () -> StateStrategy + 'static , preconditions : impl Fn (& State , & Transition) -> bool + 'static , transitions : impl Fn (& State) -> TransitionStrategy + 'static , next : impl Fn (State , & Transition) -> State + 'static ,) -> Self { Self { size , init_state : Arc :: new (init_state) , preconditions : Arc :: new (preconditions) , transitions : Arc :: new (transitions) , next : Arc :: new (next) , } } }
};
}
