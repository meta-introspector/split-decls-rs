// Generated macro for impl_204 (impl)
macro_rules! Depcrate_processimpl_204 {
() => {
// Module: crate::process
// Provides: {"impl_204"}
// Dependencies: {}
impl < T > Healthcheck for & T where T : Healthcheck , { type Status = T :: Status ; fn get_status (& self) -> Result < Self :: Status > { T :: get_status (self) } fn is_alive (& self) -> Result < bool > { T :: is_alive (self) } }
};
}
