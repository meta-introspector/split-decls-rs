// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let mut post = Post :: new () ; post . add_text ("I ate a salad for lunch today") ; assert_eq ! ("" , post . content ()) ; post . request_review () ; assert_eq ! ("" , post . content ()) ; post . approve () ; assert_eq ! ("I ate a salad for lunch today" , post . content ()) ; }
};
}
