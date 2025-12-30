// Generated macro for macro_73 (macro)
macro_rules! Depcratemacro_73 {
() => {
// Module: crate
// Provides: {"macro_73"}
// Dependencies: {}
if_hyper ! { # [cfg (test)] # [macro_use] extern crate doc_comment ; # [cfg (test)] doctest ! ("../README.md") ; pub use self :: async_impl :: { Body , Client , ClientBuilder , Request , RequestBuilder , Response , Upgraded , } ; pub use self :: proxy :: { Proxy , NoProxy } ; # [cfg (feature = "__tls")] pub use tls :: { Certificate , Identity } ; # [cfg (feature = "multipart")] pub use self :: async_impl :: multipart ; mod async_impl ; # [cfg (feature = "blocking")] pub mod blocking ; mod connect ; # [cfg (feature = "cookies")] pub mod cookie ; pub mod dns ; mod proxy ; pub mod redirect ; pub mod retry ; # [cfg (feature = "__tls")] pub mod tls ; mod util ; # [cfg (docsrs)] pub use connect :: uds :: UnixSocketProvider ; }
};
}
