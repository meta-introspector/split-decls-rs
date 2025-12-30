// Generated macro for impl_174 (impl)
macro_rules! Depcrateimpl_174 {
() => {
// Module: crate
// Provides: {"impl_174"}
// Dependencies: {}
impl < T > Log for & '_ T where T : ? Sized + Log , { fn enabled (& self , metadata : & Metadata) -> bool { (* * self) . enabled (metadata) } fn log (& self , record : & Record) { (* * self) . log (record) ; } fn flush (& self) { (* * self) . flush () ; } }
};
}
