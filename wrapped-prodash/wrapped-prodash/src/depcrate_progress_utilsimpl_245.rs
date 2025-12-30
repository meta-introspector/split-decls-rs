// Generated macro for impl_245 (impl)
macro_rules! Depcrate_progress_utilsimpl_245 {
() => {
// Module: crate::progress::utils
// Provides: {"impl_245"}
// Dependencies: {}
impl < T : NestedProgress > DoOrDiscard < T > { # [doc = " Obtain either the original [`NestedProgress`] implementation or `None`."] pub fn into_inner (self) -> Option < T > { match self { DoOrDiscard (Either :: Left (p)) => Some (p) , DoOrDiscard (Either :: Right (_)) => None , } } # [doc = " Take out the implementation of [`NestedProgress`] and replace it with [`Discard`]."] pub fn take (& mut self) -> Option < T > { let this = std :: mem :: replace (self , DoOrDiscard :: from (None)) ; match this { DoOrDiscard (Either :: Left (p)) => Some (p) , DoOrDiscard (Either :: Right (_)) => None , } } }
};
}
