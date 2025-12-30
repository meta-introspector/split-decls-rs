// Generated macro for Message (trait)
macro_rules! Depcrate_handlerMessage {
() => {
// Module: crate::handler
// Provides: {"Message"}
// Dependencies: {}
# [doc = " Represent message that can be handled by an actor."] pub trait Message { # [doc = " The type of value that this message will resolved with if it is"] # [doc = " successful."] type Result : 'static ; }
};
}
