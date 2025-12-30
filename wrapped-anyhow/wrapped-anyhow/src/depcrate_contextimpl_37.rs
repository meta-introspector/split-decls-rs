// Generated macro for impl_37 (impl)
macro_rules! Depcrate_contextimpl_37 {
() => {
// Module: crate::context
// Provides: {"impl_37"}
// Dependencies: {}
# [doc = " ```"] # [doc = " # type T = ();"] # [doc = " #"] # [doc = " use anyhow::{Context, Result};"] # [doc = ""] # [doc = " fn maybe_get() -> Option<T> {"] # [doc = "     # const IGNORE: &str = stringify! {"] # [doc = "     ..."] # [doc = "     # };"] # [doc = "     # unimplemented!()"] # [doc = " }"] # [doc = ""] # [doc = " fn demo() -> Result<()> {"] # [doc = "     let t = maybe_get().context(\"there is no T\")?;"] # [doc = "     # const IGNORE: &str = stringify! {"] # [doc = "     ..."] # [doc = "     # };"] # [doc = "     # unimplemented!()"] # [doc = " }"] # [doc = " ```"] impl < T > Context < T , Infallible > for Option < T > { fn context < C > (self , context : C) -> Result < T , Error > where C : Display + Send + Sync + 'static , { match self { Some (ok) => Ok (ok) , None => Err (Error :: construct_from_display (context , backtrace ! ())) , } } fn with_context < C , F > (self , context : F) -> Result < T , Error > where C : Display + Send + Sync + 'static , F : FnOnce () -> C , { match self { Some (ok) => Ok (ok) , None => Err (Error :: construct_from_display (context () , backtrace ! ())) , } } }
};
}
