// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let value = Rc :: new (RefCell :: new (5)) ; let a = Rc :: new (Cons (Rc :: clone (& value) , Rc :: new (Nil))) ; let b = Cons (Rc :: new (RefCell :: new (3)) , Rc :: clone (& a)) ; let c = Cons (Rc :: new (RefCell :: new (4)) , Rc :: clone (& a)) ; * value . borrow_mut () += 10 ; println ! ("a after = {a:?}") ; println ! ("b after = {b:?}") ; println ! ("c after = {c:?}") ; }
};
}
