// Generated macro for ActorFutureExt (trait)
macro_rules! Depcrate_fut_futureActorFutureExt {
() => {
// Module: crate::fut::future
// Provides: {"ActorFutureExt"}
// Dependencies: {}
pub trait ActorFutureExt < A : Actor > : ActorFuture < A > { # [doc = " Map this future's result to a different type, returning a new future of"] # [doc = " the resulting type."] fn map < F , U > (self , f : F) -> Map < Self , F > where F : FnOnce (Self :: Output , & mut A , & mut A :: Context) -> U , Self : Sized , { Map :: new (self , f) } # [doc = " Chain on a computation for when a future finished, passing the result of"] # [doc = " the future to the provided closure `f`."] fn then < F , Fut > (self , f : F) -> Then < Self , Fut , F > where F : FnOnce (Self :: Output , & mut A , & mut A :: Context) -> Fut , Fut : ActorFuture < A > , Self : Sized , { then :: new (self , f) } # [doc = " Add timeout to futures chain."] # [doc = ""] # [doc = " `Err(())` returned as a timeout error."] fn timeout (self , timeout : Duration) -> Timeout < Self > where Self : Sized , { Timeout :: new (self , timeout) } # [doc = " Wrap the future in a Box, pinning it."] # [doc = ""] # [doc = " A shortcut for wrapping in [`Box::pin`]."] fn boxed_local (self) -> LocalBoxActorFuture < A , Self :: Output > where Self : Sized + 'static , { Box :: pin (self) } }
};
}
