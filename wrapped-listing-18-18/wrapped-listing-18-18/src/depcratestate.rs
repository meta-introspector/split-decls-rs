// Generated macro for State (trait)
macro_rules! DepcrateState {
() => {
// Module: crate
// Provides: {"State"}
// Dependencies: {}
trait State { fn request_review (self : Box < Self >) -> Box < dyn State > ; fn approve (self : Box < Self >) -> Box < dyn State > ; fn content < 'a > (& self , post : & 'a Post) -> & 'a str { "" } }
};
}
