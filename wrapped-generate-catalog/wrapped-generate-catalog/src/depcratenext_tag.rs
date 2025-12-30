// Generated macro for next_tag (function)
macro_rules! Depcratenext_tag {
() => {
// Module: crate
// Provides: {"next_tag"}
// Dependencies: {}
fn next_tag < 'a > (it : & 'a mut NextSiblings < Node >) -> ElementRef < 'a > { loop { let item = it . next () . expect ("Expected a sibling") ; match item . value () { Node :: Element (_) => return ElementRef :: wrap (item) . unwrap () , _ => { } } } }
};
}
