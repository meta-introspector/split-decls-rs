// Generated macro for RUNTIME (static)
macro_rules! Depcrate_tokioRUNTIME {
() => {
// Module: crate::tokio
// Provides: {"RUNTIME"}
// Dependencies: {}
static RUNTIME : LazyLock < tokio :: runtime :: Handle > = LazyLock :: new (| | { tokio :: runtime :: Handle :: try_current () . unwrap_or_else (| _ | { let rt = tokio :: runtime :: Runtime :: new () . expect ("failed to build tokio runtime") ; let handle = rt . handle () . clone () ; std :: thread :: Builder :: new () . name ("async-global-executor/tokio" . to_string ()) . spawn (move | | { rt . block_on (futures_lite :: future :: pending :: < () > ()) ; }) . expect ("failed to spawn tokio driver thread") ; handle }) }) ;
};
}
