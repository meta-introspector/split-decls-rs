// Generated macro for impl_214 (impl)
macro_rules! Depcrateimpl_214 {
() => {
// Module: crate
// Provides: {"impl_214"}
// Dependencies: {}
# [cfg (feature = "crossbeam-channel")] impl EventHandler for crossbeam_channel :: Sender < Result < Event > > { fn handle_event (& mut self , event : Result < Event >) { let _ = self . send (event) ; } }
};
}
