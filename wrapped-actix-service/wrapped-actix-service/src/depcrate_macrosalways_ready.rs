// Generated macro for always_ready (macro)
macro_rules! Depcrate_macrosalways_ready {
() => {
// Module: crate::macros
// Provides: {"always_ready"}
// Dependencies: {}
# [doc = " An implementation of [`poll_ready`]() that always signals readiness."] # [doc = ""] # [doc = " This should only be used for basic leaf services that have no concept of un-readiness."] # [doc = " For wrapper or other service types, use [`forward_ready!`] for simple cases or write a bespoke"] # [doc = " `poll_ready` implementation."] # [doc = ""] # [doc = " [`poll_ready`]: crate::Service::poll_ready"] # [doc = ""] # [doc = " # Examples"] # [doc = " ```no_run"] # [doc = " use actix_service::Service;"] # [doc = " use futures_util::future::{ready, Ready};"] # [doc = ""] # [doc = " struct IdentityService;"] # [doc = ""] # [doc = " impl Service<u32> for IdentityService {"] # [doc = "     type Response = u32;"] # [doc = "     type Error = ();"] # [doc = "     type Future = Ready<Result<Self::Response, Self::Error>>;"] # [doc = ""] # [doc = "     actix_service::always_ready!();"] # [doc = ""] # [doc = "     fn call(&self, req: u32) -> Self::Future {"] # [doc = "         ready(Ok(req))"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [`forward_ready!`]: crate::forward_ready"] # [macro_export] macro_rules ! always_ready { () => { # [inline] fn poll_ready (& self , _ : & mut :: core :: task :: Context <'_ >,) -> :: core :: task :: Poll < Result < () , Self :: Error >> { :: core :: task :: Poll :: Ready (Ok (())) } } ; }
};
}
