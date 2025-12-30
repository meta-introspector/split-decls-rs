// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let leaf = Rc :: new (Node { value : 3 , children : RefCell :: new (vec ! []) , }) ; let branch = Rc :: new (Node { value : 5 , children : RefCell :: new (vec ! [Rc :: clone (& leaf)]) , }) ; }
};
}
