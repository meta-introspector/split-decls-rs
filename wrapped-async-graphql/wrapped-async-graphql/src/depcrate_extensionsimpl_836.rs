// Generated macro for impl_836 (impl)
macro_rules! Depcrate_extensionsimpl_836 {
() => {
// Module: crate::extensions
// Provides: {"impl_836"}
// Dependencies: {}
impl NextPrepareRequest < '_ > { # [doc = " Call the [Extension::prepare_request] function of next extension."] pub async fn run (self , ctx : & ExtensionContext < '_ > , request : Request) -> ServerResult < Request > { if let Some ((first , next)) = self . chain . split_first () { first . prepare_request (ctx , request , NextPrepareRequest { chain : next }) . await } else { Ok (request) } } }
};
}
