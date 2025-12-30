// Generated macro for impl_213 (impl)
macro_rules! Depcrateimpl_213 {
() => {
// Module: crate
// Provides: {"impl_213"}
// Dependencies: {}
impl < F > EventHandler for F where F : FnMut (Result < Event >) + Send + 'static , { fn handle_event (& mut self , event : Result < Event >) { (self) (event) ; } }
};
}
