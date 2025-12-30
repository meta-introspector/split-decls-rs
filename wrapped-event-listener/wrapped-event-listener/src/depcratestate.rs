// Generated macro for State (enum)
macro_rules! DepcrateState {
() => {
// Module: crate
// Provides: {"State"}
// Dependencies: {}
# [doc = " The state of a listener."] # [derive (PartialEq)] enum State < T > { # [doc = " The listener was just created."] Created , # [doc = " The listener has received a notification."] # [doc = ""] # [doc = " The `bool` is `true` if this was an \"additional\" notification."] Notified { # [doc = " Whether or not this is an \"additional\" notification."] additional : bool , # [doc = " The tag associated with the notification."] tag : T , } , # [doc = " A task is waiting for a notification."] Task (Task) , # [doc = " Empty hole used to replace a notified listener."] NotifiedTaken , }
};
}
