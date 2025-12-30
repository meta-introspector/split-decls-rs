// Generated macro for impl_204 (impl)
macro_rules! Depcrate_utilsimpl_204 {
() => {
// Module: crate::utils
// Provides: {"impl_204"}
// Dependencies: {}
impl FromStr for Version { type Err = () ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if let Some (s) = s . strip_prefix ("0.") && let Some ((major , minor)) = s . split_once ('.') && let Ok (major) = major . parse () && let Ok (minor) = minor . parse () { Ok (Self { major , minor }) } else { Err (()) } } }
};
}
