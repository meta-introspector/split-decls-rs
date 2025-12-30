// Generated macro for impl_58 (impl)
macro_rules! Depcrate_inputimpl_58 {
() => {
// Module: crate::input
// Provides: {"impl_58"}
// Dependencies: {}
impl ReleaseChannel { pub fn as_str (self) -> & 'static str { match self { ReleaseChannel :: Stable => "stable" , ReleaseChannel :: Beta => "beta" , ReleaseChannel :: Nightly => "nightly" , } } # [allow (clippy :: should_implement_trait)] pub fn from_str (str : & str) -> Option < Self > { Some (match str { "" | "stable" => ReleaseChannel :: Stable , "nightly" => ReleaseChannel :: Nightly , _ if str . starts_with ("beta") => ReleaseChannel :: Beta , _ => return None , }) } }
};
}
