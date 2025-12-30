// Generated macro for ActorTryFuture (trait)
macro_rules! Depcrate_fut_try_futureActorTryFuture {
() => {
// Module: crate::fut::try_future
// Provides: {"ActorTryFuture"}
// Dependencies: {}
# [doc = " A convenience for actor futures that return `Result` values that includes"] # [doc = " a variety of adapters tailored to such actor futures."] pub trait ActorTryFuture < A : Actor > : ActorFuture < A > + private_try_act_future :: Sealed < A > { # [doc = " The type of successful values yielded by this actor future"] type Ok ; # [doc = " The type of failures yielded by this actor  future"] type Error ; # [doc = " Poll this `ActorTryFuture` as if it were a `ActorFuture`."] # [doc = ""] # [doc = " This method is a stopgap for a compiler limitation that prevents us from"] # [doc = " directly inheriting from the `ActorFuture` trait; in the actor future it"] # [doc = " won't be needed."] fn try_poll (self : Pin < & mut Self > , srv : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Result < Self :: Ok , Self :: Error > > ; }
};
}
