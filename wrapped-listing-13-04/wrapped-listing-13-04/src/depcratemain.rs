// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let list = vec ! [1 , 2 , 3] ; println ! ("Before defining closure: {list:?}") ; let only_borrows = | | println ! ("From closure: {list:?}") ; println ! ("Before calling closure: {list:?}") ; only_borrows () ; println ! ("After calling closure: {list:?}") ; }
};
}
