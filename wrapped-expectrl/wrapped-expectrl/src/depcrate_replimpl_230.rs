// Generated macro for impl_230 (impl)
macro_rules! Depcrate_replimpl_230 {
() => {
// Module: crate::repl
// Provides: {"impl_230"}
// Dependencies: {}
impl < S > Healthcheck for ReplSession < S > where S : Healthcheck , { type Status = S :: Status ; fn get_status (& self) -> io :: Result < Self :: Status > { self . get_session () . get_status () } fn is_alive (& self) -> io :: Result < bool > { self . get_session () . is_alive () } }
};
}
