// Generated macro for impl_37 (impl)
macro_rules! Depcrate_errorimpl_37 {
() => {
// Module: crate::error
// Provides: {"impl_37"}
// Dependencies: {}
impl From < & '_ std :: path :: Path > for ImageFormatHint { fn from (path : & '_ std :: path :: Path) -> Self { match path . extension () { Some (ext) => ImageFormatHint :: PathExtension (ext . into ()) , None => ImageFormatHint :: Unknown , } } }
};
}
