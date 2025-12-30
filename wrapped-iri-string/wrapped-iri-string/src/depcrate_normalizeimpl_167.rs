// Generated macro for impl_167 (impl)
macro_rules! Depcrate_normalizeimpl_167 {
() => {
// Module: crate::normalize
// Provides: {"impl_167"}
// Dependencies: {}
impl NormalizationInput < '_ > { # [doc = " Checks if the path is normalizable by RFC 3986 algorithm."] # [doc = ""] # [doc = " Returns `Ok(())` when normalizable, returns `Err(_)` if not."] pub (crate) fn ensure_rfc3986_normalizable (& self) -> Result < () , Error > { if self . authority . is_some () { return Ok (()) ; } match self . path { Path :: Done (_) => Ok (()) , Path :: NeedsProcessing (path) => path . ensure_rfc3986_normalizable_with_authority_absent () , } } }
};
}
