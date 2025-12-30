// Generated macro for impl_626 (impl)
macro_rules! Depcrate_versionimpl_626 {
() => {
// Module: crate::version
// Provides: {"impl_626"}
// Dependencies: {}
impl RustcVersion { pub const CURRENT : Self = current_rustc_version ! () ; pub fn current_overridable () -> Self { * CURRENT_OVERRIDABLE . get_or_init (| | { if let Ok (override_var) = std :: env :: var ("RUSTC_OVERRIDE_VERSION_STRING") && let Some (override_) = Self :: parse_str (& override_var) { override_ } else { Self :: CURRENT } }) } fn parse_str (value : & str) -> Option < Self > { let mut components = value . split ('-') . next () . unwrap () . splitn (3 , '.') ; let major = components . next () ? . parse () . ok () ? ; let minor = components . next () ? . parse () . ok () ? ; let patch = components . next () . unwrap_or ("0") . parse () . ok () ? ; Some (RustcVersion { major , minor , patch }) } }
};
}
