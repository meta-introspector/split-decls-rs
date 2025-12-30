// Generated macro for add (function)
macro_rules! Depcrate_iter_sumadd {
() => {
// Module: crate::iter::sum
// Provides: {"add"}
// Dependencies: {}
fn add < T : Sum > (left : T , right : T) -> T { [left , right] . into_iter () . sum () }
};
}
