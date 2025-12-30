// Generated macro for impl_253 (impl)
macro_rules! Depcrate_client_git_async_ioimpl_253 {
() => {
// Module: crate::client::git::async_io
// Provides: {"impl_253"}
// Dependencies: {}
impl < R , W > Connection < R , W > { # [doc = " Optionally set the URL to be returned when asked for it if `Some` or calculate a default for `None`."] # [doc = ""] # [doc = " The URL is required as parameter for authentication helpers which are called in transports"] # [doc = " that support authentication. Even though plain git transports don't support that, this"] # [doc = " may well be the case in custom transports."] pub fn custom_url (mut self , url : Option < BString >) -> Self { self . state . custom_url = url ; self } # [doc = " Return the inner reader and writer"] pub fn into_inner (self) -> (R , W) { (self . line_provider . into_inner () , self . writer) } }
};
}
