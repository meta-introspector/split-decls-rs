// Generated macro for impl_56 (impl)
macro_rules! Depcrate_eventsimpl_56 {
() => {
// Module: crate::events
// Provides: {"impl_56"}
// Dependencies: {}
impl std :: fmt :: Display for EventCategory { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { let v = match self { EventCategory :: Connectivity => "connectivity" , EventCategory :: Security => "security" , EventCategory :: Transport => "transport" , EventCategory :: Recovery => "recovery" , EventCategory :: Http => "http" , EventCategory :: Qpack => "qpack" , EventCategory :: Error => "error" , EventCategory :: Warning => "warning" , EventCategory :: Info => "info" , EventCategory :: Debug => "debug" , EventCategory :: Verbose => "verbose" , EventCategory :: Simulation => "simulation" , } ; write ! (f , "{v}" ,) } }
};
}
