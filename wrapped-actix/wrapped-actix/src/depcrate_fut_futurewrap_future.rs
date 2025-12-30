// Generated macro for wrap_future (function)
macro_rules! Depcrate_fut_futurewrap_future {
() => {
// Module: crate::fut::future
// Provides: {"wrap_future"}
// Dependencies: {}
# [doc = " Converts normal future into [`ActorFuture`], allowing its processing to"] # [doc = " use the actor's state."] # [doc = ""] # [doc = " See the documentation for [`ActorFuture`] for a practical example involving both"] # [doc = " [`wrap_future`] and [`ActorFuture`]"] pub fn wrap_future < F , A > (f : F) -> FutureWrap < F , A > where F : Future , A : Actor , { FutureWrap { fut : f , _act : PhantomData , } }
};
}
