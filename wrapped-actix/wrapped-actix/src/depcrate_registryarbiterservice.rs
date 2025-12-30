// Generated macro for ArbiterService (trait)
macro_rules! Depcrate_registryArbiterService {
() => {
// Module: crate::registry
// Provides: {"ArbiterService"}
// Dependencies: {}
# [doc = " Trait defines arbiter's service."] # [allow (unused_variables)] pub trait ArbiterService : Actor < Context = Context < Self > > + Supervised + Default { # [doc = " Construct and start arbiter service"] fn start_service () -> Addr < Self > { Supervisor :: start (| ctx | { let mut act = Self :: default () ; act . service_started (ctx) ; act }) } # [doc = " Method is called during service initialization."] fn service_started (& mut self , ctx : & mut Context < Self >) { } # [doc = " Get actor's address from arbiter registry"] fn from_registry () -> Addr < Self > { AREG . with (| reg | reg . get_or_start_default ()) } }
};
}
