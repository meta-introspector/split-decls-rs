// Generated macro for impl_176 (impl)
macro_rules! Depcrateimpl_176 {
() => {
// Module: crate
// Provides: {"impl_176"}
// Dependencies: {}
# [cfg (feature = "std")] impl < T > Log for std :: sync :: Arc < T > where T : ? Sized + Log , { fn enabled (& self , metadata : & Metadata) -> bool { self . as_ref () . enabled (metadata) } fn log (& self , record : & Record) { self . as_ref () . log (record) ; } fn flush (& self) { self . as_ref () . flush () ; } }
};
}
