// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let leaf = Rc :: new (Node { value : 3 , parent : RefCell :: new (Weak :: new ()) , children : RefCell :: new (vec ! []) , }) ; println ! ("leaf parent = {:?}" , leaf . parent . borrow () . upgrade ()) ; let branch = Rc :: new (Node { value : 5 , parent : RefCell :: new (Weak :: new ()) , children : RefCell :: new (vec ! [Rc :: clone (& leaf)]) , }) ; * leaf . parent . borrow_mut () = Rc :: downgrade (& branch) ; println ! ("leaf parent = {:?}" , leaf . parent . borrow () . upgrade ()) ; }
};
}
