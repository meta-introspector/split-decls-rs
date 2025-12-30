// Generated macro for impl_1328 (impl)
macro_rules! Depcrate_optionimpl_1328 {
() => {
// Module: crate::option
// Provides: {"impl_1328"}
// Dependencies: {}
impl < T : Send > Producer for OptionProducer < T > { type Item = T ; type IntoIter = std :: option :: IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { self . opt . into_iter () } fn split_at (self , index : usize) -> (Self , Self) { debug_assert ! (index <= 1) ; let none = OptionProducer { opt : None } ; if index == 0 { (none , self) } else { (self , none) } } }
};
}
