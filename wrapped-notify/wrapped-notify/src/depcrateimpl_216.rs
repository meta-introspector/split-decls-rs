// Generated macro for impl_216 (impl)
macro_rules! Depcrateimpl_216 {
() => {
// Module: crate
// Provides: {"impl_216"}
// Dependencies: {}
impl EventHandler for std :: sync :: mpsc :: Sender < Result < Event > > { fn handle_event (& mut self , event : Result < Event >) { let _ = self . send (event) ; } }
};
}
