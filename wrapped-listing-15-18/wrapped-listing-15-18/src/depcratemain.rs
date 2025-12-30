// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let a = Rc :: new (Cons (5 , Rc :: new (Cons (10 , Rc :: new (Nil))))) ; let b = Cons (3 , Rc :: clone (& a)) ; let c = Cons (4 , Rc :: clone (& a)) ; }
};
}
