// Generated macro for impl_248 (impl)
macro_rules! Depcrate_session_async_sessionimpl_248 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_248"}
// Dependencies: {}
impl < P , S > Healthcheck for Session < P , S > where P : Healthcheck , { type Status = P :: Status ; # [doc = " Verifies whether process is still alive."] fn is_alive (& self) -> io :: Result < bool > { P :: is_alive (self . get_process ()) } fn get_status (& self) -> io :: Result < Self :: Status > { P :: get_status (self . get_process ()) } }
};
}
