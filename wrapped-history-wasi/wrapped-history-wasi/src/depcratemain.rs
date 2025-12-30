// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let history = MemoryHistory :: new () ; println ! ("Current path: {}" , history . location () . path ()) ; assert_eq ! (history . location () . path () , "/") ; history . push ("/home") ; println ! ("Current path: {}" , history . location () . path ()) ; assert_eq ! (history . location () . path () , "/home") ; history . push ("/about") ; println ! ("Current path: {}" , history . location () . path ()) ; assert_eq ! (history . location () . path () , "/about") ; history . push ("/contact") ; println ! ("Current path: {}" , history . location () . path ()) ; assert_eq ! (history . location () . path () , "/contact") ; history . go (- 2) ; println ! ("Current path: {}" , history . location () . path ()) ; assert_eq ! (history . location () . path () , "/home") ; }
};
}
