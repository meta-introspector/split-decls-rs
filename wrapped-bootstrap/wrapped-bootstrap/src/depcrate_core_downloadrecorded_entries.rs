// Generated macro for recorded_entries (function)
macro_rules! Depcrate_core_downloadrecorded_entries {
() => {
// Module: crate::core::download
// Provides: {"recorded_entries"}
// Dependencies: {}
fn recorded_entries (dst : & Path , pattern : & str) -> Option < BufWriter < File > > { let name = if pattern == "rustc-dev" { ".rustc-dev-contents" } else if pattern . starts_with ("rust-std") { ".rust-std-contents" } else { return None ; } ; Some (BufWriter :: new (t ! (File :: create (dst . join (name))))) }
};
}
