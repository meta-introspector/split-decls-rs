// Generated macro for impl_656 (impl)
macro_rules! Depcrate_diffimpl_656 {
() => {
// Module: crate::diff
// Provides: {"impl_656"}
// Dependencies: {}
# [doc = " Lifecycle"] impl Options { # [cfg (feature = "blob-diff")] pub (crate) fn from_configuration (config : & crate :: config :: Cache) -> Result < Self , options :: init :: Error > { Ok (Options { location : Some (Location :: Path) , rewrites : { let (rewrites , is_configured) = config . diff_renames () ? ; if is_configured { rewrites } else { Some (Default :: default ()) } } , }) } }
};
}
