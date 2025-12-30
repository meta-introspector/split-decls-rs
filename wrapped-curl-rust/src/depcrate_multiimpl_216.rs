// Generated macro for impl_216 (impl)
macro_rules! Depcrate_multiimpl_216 {
() => {
// Module: crate::multi
// Provides: {"impl_216"}
// Dependencies: {}
impl DetachGuard { # [doc = " Detach the referenced easy handle from its multi handle manually."] # [doc = " Subsequent calls to this method will have no effect."] fn detach (& mut self) -> Result < () , MultiError > { if ! self . easy . is_null () { unsafe { cvt (curl_sys :: curl_multi_remove_handle (self . multi . handle , self . easy ,)) ? } self . easy = ptr :: null_mut () ; } Ok (()) } }
};
}
