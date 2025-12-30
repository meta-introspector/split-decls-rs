// Generated macro for impl_111 (impl)
macro_rules! Depcrate_fetch_refmapimpl_111 {
() => {
// Module: crate::fetch::refmap
// Provides: {"impl_111"}
// Dependencies: {}
impl SpecIndex { # [doc = " Depending on our index variant, get the index either from `refspecs` or from `extra_refspecs` for `Implicit` variants."] pub fn get < 'a > (self , refspecs : & 'a [gix_refspec :: RefSpec] , extra_refspecs : & 'a [gix_refspec :: RefSpec] ,) -> Option < & 'a gix_refspec :: RefSpec > { match self { SpecIndex :: ExplicitInRemote (idx) => refspecs . get (idx) , SpecIndex :: Implicit (idx) => extra_refspecs . get (idx) , } } # [doc = " If this is an `Implicit` variant, return its index."] pub fn implicit_index (self) -> Option < usize > { match self { SpecIndex :: Implicit (idx) => Some (idx) , SpecIndex :: ExplicitInRemote (_) => None , } } }
};
}
