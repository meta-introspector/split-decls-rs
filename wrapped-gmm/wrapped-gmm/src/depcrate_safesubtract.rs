// Generated macro for subtract (function)
macro_rules! Depcrate_safesubtract {
() => {
// Module: crate::safe
// Provides: {"subtract"}
// Dependencies: {}
fn subtract (d : usize , x : & [f64] , y : & [f64] , out : & mut [f64]) { assert ! (x . len () >= d) ; assert ! (y . len () >= d) ; assert ! (out . len () >= d) ; for i in 0 .. d { out [i] = x [i] - y [i] ; } }
};
}
