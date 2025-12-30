// Generated macro for forward_ready (macro)
macro_rules! Depcrate_macrosforward_ready {
() => {
// Module: crate::macros
// Provides: {"forward_ready"}
// Dependencies: {}
# [doc = " An implementation of [`poll_ready`] that forwards readiness checks to a"] # [doc = " named struct field."] # [doc = ""] # [doc = " Tuple structs are not supported."] # [doc = ""] # [doc = " [`poll_ready`]: crate::Service::poll_ready"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use actix_service::Service;"] # [doc = " use futures_util::future::{ready, Ready};"] # [doc = ""] # [doc = " struct WrapperService<S> {"] # [doc = "     inner: S,"] # [doc = " }"] # [doc = ""] # [doc = " impl<S> Service<()> for WrapperService<S>"] # [doc = " where"] # [doc = "     S: Service<()>,"] # [doc = " {"] # [doc = "     type Response = S::Response;"] # [doc = "     type Error = S::Error;"] # [doc = "     type Future = S::Future;"] # [doc = ""] # [doc = "     actix_service::forward_ready!(inner);"] # [doc = ""] # [doc = "     fn call(&self, req: ()) -> Self::Future {"] # [doc = "         self.inner.call(req)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! forward_ready { ($ field : ident) => { # [inline] fn poll_ready (& self , cx : & mut :: core :: task :: Context <'_ >,) -> :: core :: task :: Poll < Result < () , Self :: Error >> { self .$ field . poll_ready (cx) . map_err (:: core :: convert :: Into :: into) } } ; }
};
}
