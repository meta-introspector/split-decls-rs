// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let a = Rc :: new (Cons (5 , RefCell :: new (Rc :: new (Nil)))) ; println ! ("a initial rc count = {}" , Rc :: strong_count (& a)) ; println ! ("a next item = {:?}" , a . tail ()) ; let b = Rc :: new (Cons (10 , RefCell :: new (Rc :: clone (& a)))) ; println ! ("a rc count after b creation = {}" , Rc :: strong_count (& a)) ; println ! ("b initial rc count = {}" , Rc :: strong_count (& b)) ; println ! ("b next item = {:?}" , b . tail ()) ; if let Some (link) = a . tail () { * link . borrow_mut () = Rc :: clone (& b) ; } println ! ("b rc count after changing a = {}" , Rc :: strong_count (& b)) ; println ! ("a rc count after changing a = {}" , Rc :: strong_count (& a)) ; }
};
}
