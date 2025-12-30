// Generated macro for impl_832 (impl)
macro_rules! Depcrate_extensionsimpl_832 {
() => {
// Module: crate::extensions
// Provides: {"impl_832"}
// Dependencies: {}
impl NextRequest < '_ > { # [doc = " Call the [Extension::request] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ >) -> Response { if let Some ((first , next)) = self . chain . split_first () { first . request (ctx , NextRequest { chain : next , request_fut : self . request_fut , } ,) . await } else { self . request_fut . await } } }
};
}
