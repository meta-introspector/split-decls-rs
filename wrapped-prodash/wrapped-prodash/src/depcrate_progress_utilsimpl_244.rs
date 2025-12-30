// Generated macro for impl_244 (impl)
macro_rules! Depcrate_progress_utilsimpl_244 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_244"}
// Dependencies: {}
impl < T > From < Option < T > > for DoOrDiscard < T > where T : NestedProgress , { fn from (p : Option < T >) -> Self { match p { Some (p) => DoOrDiscard (Either :: Left (p)) , None => DoOrDiscard (Either :: Right (Discard)) , } } }
};
}
