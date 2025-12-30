// Generated macro for impl_123 (impl)
macro_rules! Depcrate_fetch_typesimpl_123 {
() => {
// Module: crate::fetch::types
// Provides: {"impl_123"}
// Dependencies: {}
impl Tags { # [doc = " Obtain a refspec that determines whether or not to fetch all tags, depending on this variant."] # [doc = ""] # [doc = " The returned refspec is the default refspec for tags, but won't overwrite local tags ever."] # [cfg (feature = "fetch")] pub fn to_refspec (& self) -> Option < gix_refspec :: RefSpecRef < 'static > > { match self { Tags :: All | Tags :: Included => Some (gix_refspec :: parse ("refs/tags/*:refs/tags/*" . into () , gix_refspec :: parse :: Operation :: Fetch) . expect ("valid") ,) , Tags :: None => None , } } }
};
}
