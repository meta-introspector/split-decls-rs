// Generated macro for WrapFuture (trait)
macro_rules! Depcrate_fut_futureWrapFuture {
() => {
// Module: crate::fut::future
// Provides: {"WrapFuture"}
// Dependencies: {}
# [doc = " Helper trait that allows conversion of normal future into [`ActorFuture`]"] pub trait WrapFuture < A > where A : Actor , { # [doc = " The future that this type can be converted into."] type Future : ActorFuture < A > ; # [deprecated (since = "0.11.0" , note = "Please use WrapFuture::into_actor")] # [doc (hidden)] fn actfuture (self) -> Self :: Future ; # [doc = " Convert normal future to a [`ActorFuture`]"] fn into_actor (self , a : & A) -> Self :: Future ; }
};
}
