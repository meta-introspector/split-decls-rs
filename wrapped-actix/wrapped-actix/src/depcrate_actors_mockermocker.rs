// Generated macro for Mocker (struct)
macro_rules! Depcrate_actors_mockerMocker {
() => {
// Module: crate::actors::mocker
// Provides: {"Mocker"}
// Dependencies: {}
# [doc = " This actor is able to wrap another actor and accept all the messages the"] # [doc = " wrapped actor can, passing it to a closure which can mock the response of"] # [doc = " the actor."] # [allow (clippy :: type_complexity)] pub struct Mocker < T : Sized + Unpin + 'static > { phantom : PhantomData < T > , mock : Box < dyn FnMut (Box < dyn Any > , & mut Context < Mocker < T > >) -> Box < dyn Any > > , }
};
}
