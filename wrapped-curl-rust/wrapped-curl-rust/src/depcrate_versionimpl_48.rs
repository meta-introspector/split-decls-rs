// Generated macro for impl_48 (impl)
macro_rules! Depcrate_versionimpl_48 {
() => {
// Module: crate::version
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a > Iterator for Protocols < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { unsafe { if (* self . cur) . is_null () { return None ; } let ret = crate :: opt_str (* self . cur) . unwrap () ; self . cur = self . cur . offset (1) ; Some (ret) } } }
};
}
