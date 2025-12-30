// Generated macro for impl_1636 (impl)
macro_rules! Depcrateimpl_1636 {
() => {
// Module: crate
// Provides: {"impl_1636"}
// Dependencies: {}
impl Crate { fn local_path (& self , build : & Build) -> PathBuf { self . path . strip_prefix (& build . config . src) . unwrap () . into () } }
};
}
