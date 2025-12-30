// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let c = CustomSmartPointer { data : String :: from ("some data") , } ; println ! ("CustomSmartPointer created") ; drop (c) ; println ! ("CustomSmartPointer dropped before the end of main") ; }
};
}
