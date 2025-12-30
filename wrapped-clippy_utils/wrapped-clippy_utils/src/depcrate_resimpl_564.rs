// Generated macro for impl_564 (impl)
macro_rules! Depcrate_resimpl_564 {
() => {
// Module: crate::res
// Provides: {"impl_564"}
// Dependencies: {}
impl < 'a , T : MaybeResPath < 'a > > MaybeResPath < 'a > for Option < T > { # [inline] fn opt_res_path (self) -> OptResPath < 'a > { match self { Some (x) => T :: opt_res_path (x) , None => (None , None) , } } # [inline] fn basic_res (self) -> & 'a Res { self . map_or (& Res :: Err , T :: basic_res) } }
};
}
