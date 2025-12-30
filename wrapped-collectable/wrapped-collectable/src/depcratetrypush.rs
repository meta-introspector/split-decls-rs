// Generated macro for TryPush (trait)
macro_rules! DepcrateTryPush {
() => {
// Module: crate
// Provides: {"TryPush"}
// Dependencies: {}
# [doc = " Try to push an element onto a collection"] pub trait TryPush < T > { # [doc = " Try to push an element onto a collection."] # [doc = ""] # [doc = " Returns the original element if it's full."] fn try_push (& mut self , item : T) -> Result < () , T > ; }
};
}
