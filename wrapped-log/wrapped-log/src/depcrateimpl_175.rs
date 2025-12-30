// Generated macro for impl_175 (impl)
macro_rules! Depcrateimpl_175 {
() => {
// Module: crate
// Provides: {"impl_175"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T > Log for std :: boxed :: Box < T > where T : ? Sized + Log , { fn enabled (& self , metadata : & Metadata) -> bool { self . as_ref () . enabled (metadata) } fn log (& self , record : & Record) { self . as_ref () . log (record) ; } fn flush (& self) { self . as_ref () . flush () ; } }
};
}
