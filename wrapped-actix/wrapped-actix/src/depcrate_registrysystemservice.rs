// Generated macro for SystemService (trait)
macro_rules! Depcrate_registrySystemService {
() => {
// Module: crate::registry
// Provides: {"SystemService"}
// Dependencies: {}
# [doc = " Trait defines system's service."] # [allow (unused_variables)] pub trait SystemService : Actor < Context = Context < Self > > + Supervised + Default { # [doc = " Construct and start system service"] fn start_service (wrk : & ArbiterHandle) -> Addr < Self > { Supervisor :: start_in_arbiter (wrk , | ctx | { let mut act = Self :: default () ; act . service_started (ctx) ; act }) } # [doc = " Method is called during service initialization."] fn service_started (& mut self , ctx : & mut Context < Self >) { } # [doc = " Get actor's address from system registry"] fn from_registry () -> Addr < Self > { let sys = System :: current () ; let mut sreg = SREG . lock () ; let reg = sreg . entry (sys . id ()) . or_insert_with (| | SystemRegistry :: new (sys . arbiter () . clone ())) ; if let Some (addr) = reg . registry . get (& TypeId :: of :: < Self > ()) { if let Some (addr) = addr . downcast_ref :: < Addr < Self > > () { return addr . clone () ; } } let addr = Self :: start_service (System :: current () . arbiter ()) ; reg . registry . insert (TypeId :: of :: < Self > () , Box :: new (addr . clone ())) ; addr } }
};
}
