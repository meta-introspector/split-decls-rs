// Generated macro for impl_215 (impl)
macro_rules! Depcrateimpl_215 {
() => {
// Module: crate
// Provides: {"impl_215"}
// Dependencies: {}
# [cfg (feature = "flume")] impl EventHandler for flume :: Sender < Result < Event > > { fn handle_event (& mut self , event : Result < Event >) { let _ = self . send (event) ; } }
};
}
