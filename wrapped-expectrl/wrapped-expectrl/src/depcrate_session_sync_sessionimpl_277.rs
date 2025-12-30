// Generated macro for impl_277 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_277 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_277"}
// Dependencies: {}
impl < P , S > Healthcheck for Session < P , S > where P : Healthcheck , { type Status = P :: Status ; fn get_status (& self) -> io :: Result < Self :: Status > { self . get_process () . get_status () } fn is_alive (& self) -> io :: Result < bool > { self . get_process () . is_alive () } }
};
}
