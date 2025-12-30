// Generated macro for impl_834 (impl)
macro_rules! Depcrate_extensionsimpl_834 {
() => {
// Module: crate::extensions
// Provides: {"impl_834"}
// Dependencies: {}
impl NextSubscribe < '_ > { # [doc = " Call the [Extension::subscribe] function of next extension."] pub fn run < 's > (self , ctx : & ExtensionContext < '_ > , stream : BoxStream < 's , Response > ,) -> BoxStream < 's , Response > { if let Some ((first , next)) = self . chain . split_first () { first . subscribe (ctx , stream , NextSubscribe { chain : next }) } else { stream } } }
};
}
