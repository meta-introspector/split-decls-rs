// Generated macro for impl_3068 (impl)
macro_rules! Depcrate_remove_constant_phisimpl_3068 {
() => {
// Module: crate::remove_constant_phis
// Provides: {"impl_3068"}
// Dependencies: {}
impl AbstractValue { fn join (self , other : AbstractValue) -> AbstractValue { match (self , other) { (AbstractValue :: None , p2) => p2 , (p1 , AbstractValue :: None) => p1 , (AbstractValue :: Many , _p2) => AbstractValue :: Many , (_p1 , AbstractValue :: Many) => AbstractValue :: Many , (AbstractValue :: One (v1) , AbstractValue :: One (v2)) => { if v1 == v2 { AbstractValue :: One (v1) } else { AbstractValue :: Many } } } } fn is_one (self) -> bool { matches ! (self , AbstractValue :: One (_)) } }
};
}
