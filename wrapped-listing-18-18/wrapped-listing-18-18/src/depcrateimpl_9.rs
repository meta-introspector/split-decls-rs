// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl State for Published { fn request_review (self : Box < Self >) -> Box < dyn State > { self } fn approve (self : Box < Self >) -> Box < dyn State > { self } fn content < 'a > (& self , post : & 'a Post) -> & 'a str { & post . content } }
};
}
