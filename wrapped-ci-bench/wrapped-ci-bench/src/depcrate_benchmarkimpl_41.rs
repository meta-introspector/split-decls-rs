// Generated macro for impl_41 (impl)
macro_rules! Depcrate_benchmarkimpl_41 {
() => {
// Module: crate::benchmark
// Provides: {"impl_41"}
// Dependencies: {}
impl ResumptionKind { pub const ALL : & 'static [Self] = & [Self :: No , Self :: SessionId , Self :: Tickets] ; # [doc = " Returns a user-facing label that identifies the resumption kind"] pub fn label (& self) -> & 'static str { match * self { Self :: No => "no_resume" , Self :: SessionId => "session_id" , Self :: Tickets => "tickets" , } } }
};
}
