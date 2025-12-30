// Generated macro for impl_2 (impl)
macro_rules! Depcrateimpl_2 {
() => {
// Module: crate
// Provides: {"impl_2"}
// Dependencies: {}
impl Post { pub fn new () -> Post { Post { state : Some (Box :: new (Draft { })) , content : String :: new () , } } pub fn add_text (& mut self , text : & str) { self . content . push_str (text) ; } pub fn content (& self) -> & str { "" } pub fn request_review (& mut self) { if let Some (s) = self . state . take () { self . state = Some (s . request_review ()) } } pub fn approve (& mut self) { if let Some (s) = self . state . take () { self . state = Some (s . approve ()) } } }
};
}
