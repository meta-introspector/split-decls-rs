// Generated macro for mul (function)
macro_rules! Depcrate_iter_productmul {
() => {
// Module: crate::iter::product
// Provides: {"mul"}
// Dependencies: {}
fn mul < T : Product > (left : T , right : T) -> T { [left , right] . into_iter () . product () }
};
}
