// Generated macro for compat (module)
macro_rules! Depcratecompat {
() => {
// Module: crate
// Provides: {"compat"}
// Dependencies: {}
# [cfg (feature = "compat")] # [cfg_attr (docsrs , doc (cfg (feature = "compat")))] pub mod compat { # ! [doc = " Interop between `futures` 0.1 and 0.3."] # ! [doc = ""] # ! [doc = " This module is only available when the `compat` feature of this"] # ! [doc = " library is activated."] pub use futures_util :: compat :: { Compat , Compat01As03 , Compat01As03Sink , CompatSink , Executor01As03 , Executor01CompatExt , Executor01Future , Future01CompatExt , Sink01CompatExt , Stream01CompatExt , } ; # [cfg (feature = "io-compat")] # [cfg_attr (docsrs , doc (cfg (feature = "io-compat")))] pub use futures_util :: compat :: { AsyncRead01CompatExt , AsyncWrite01CompatExt } ; }
};
}
