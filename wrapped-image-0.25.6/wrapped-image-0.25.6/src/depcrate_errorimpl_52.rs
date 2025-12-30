// Generated macro for impl_52 (impl)
macro_rules! Depcrate_errorimpl_52 {
() => {
// Module: crate::error
// Provides: {"impl_52"}
// Dependencies: {}
impl fmt :: Display for ImageFormatHint { fn fmt (& self , fmt : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match self { ImageFormatHint :: Exact (format) => write ! (fmt , "{format:?}") , ImageFormatHint :: Name (name) => write ! (fmt , "`{name}`") , ImageFormatHint :: PathExtension (ext) => write ! (fmt , "`.{ext:?}`") , ImageFormatHint :: Unknown => write ! (fmt , "`Unknown`") , } } }
};
}
