// Generated macro for private_try_act_future (module)
macro_rules! Depcrate_fut_try_futureprivate_try_act_future {
() => {
// Module: crate::fut::try_future
// Provides: {"private_try_act_future"}
// Dependencies: {}
mod private_try_act_future { use super :: { Actor , ActorFuture } ; pub trait Sealed < A > { } impl < A , F , T , E > Sealed < A > for F where A : Actor , F : ? Sized + ActorFuture < A , Output = Result < T , E > > , { } }
};
}
