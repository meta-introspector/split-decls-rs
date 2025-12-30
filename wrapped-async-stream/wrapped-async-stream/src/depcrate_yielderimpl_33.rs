// Generated macro for impl_33 (impl)
macro_rules! Depcrate_yielderimpl_33 {
() => {
// Module: crate::yielder
// Provides: {"impl_33"}
// Dependencies: {}
impl < T > Sender < T > { pub fn send (& mut self , value : T) -> impl Future < Output = () > { Send { value : Some (value) } } }
};
}
