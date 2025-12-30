// Generated macro for impl_220 (impl)
macro_rules! Depcrate_writeimpl_220 {
() => {
// Module: crate::write
// Provides: {"impl_220"}
// Dependencies: {}
impl State { fn detect_required_version (& self) -> Version { self . entries . iter () . find_map (| e | e . flags . contains (entry :: Flags :: EXTENDED) . then_some (Version :: V3)) . unwrap_or (Version :: V2) } }
};
}
