// Generated macro for qtimesx (function)
macro_rules! Depcrate_safeqtimesx {
() => {
// Module: crate::safe
// Provides: {"qtimesx"}
// Dependencies: {}
fn qtimesx (d : usize , q_diag : & [f64] , ltri : & [f64] , x : & [f64] , out : & mut [f64]) { assert ! (out . len () >= d) ; assert ! (q_diag . len () >= d) ; assert ! (x . len () >= d) ; for i in 0 .. d { out [i] = q_diag [i] * x [i] ; } for i in 0 .. d { let mut lparamsidx = i * (2 * d - i - 1) / 2 ; for j in i + 1 .. d { out [j] = out [j] + ltri [lparamsidx] * x [i] ; lparamsidx += 1 ; } } }
};
}
