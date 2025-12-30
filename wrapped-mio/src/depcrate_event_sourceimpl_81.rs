// Generated macro for impl_81 (impl)
macro_rules! Depcrate_event_sourceimpl_81 {
() => {
// Module: crate::event::source
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > Source for Box < T > where T : Source + ? Sized , { fn register (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { (* * self) . register (registry , token , interests) } fn reregister (& mut self , registry : & Registry , token : Token , interests : Interest ,) -> io :: Result < () > { (* * self) . reregister (registry , token , interests) } fn deregister (& mut self , registry : & Registry) -> io :: Result < () > { (* * self) . deregister (registry) } }
};
}
