// Generated macro for impl_12 (impl)
macro_rules! Depcrate_path_extimpl_12 {
() => {
// Module: crate::path_ext
// Provides: {"impl_12"}
// Dependencies: {}
impl fmt :: Display for RelativeToError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { match self . kind { RelativeToErrorKind :: NonUtf8 => "path contains non-utf8 component" . fmt (fmt) , RelativeToErrorKind :: PrefixMismatch => { "paths contain different absolute prefixes" . fmt (fmt) } RelativeToErrorKind :: AmbiguousTraversal => { "path traversal cannot be determined" . fmt (fmt) } RelativeToErrorKind :: IllegalComponent => "path contains illegal components" . fmt (fmt) , } } }
};
}
