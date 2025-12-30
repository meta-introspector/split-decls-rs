// Generated macro for impl_171 (impl)
macro_rules! Depcrateimpl_171 {
() => {
// Module: crate
// Provides: {"impl_171"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T > Log for std :: boxed :: Box < T > where T : ? Sized + Log , { fn enabled (& self , metadata : & Metadata) -> bool { self . as_ref () . enabled (metadata) } fn log (& self , record : & Record) { self . as_ref () . log (record) ; } fn flush (& self) { self . as_ref () . flush () ; } }
};
}
