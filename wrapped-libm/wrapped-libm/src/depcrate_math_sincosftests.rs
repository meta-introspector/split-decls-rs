// Generated macro for tests (module)
macro_rules! Depcrate_math_sincosftests {
() => {
// Module: crate::math::sincosf
// Provides: {"tests"}
// Dependencies: {}
# [cfg (not (target_arch = "powerpc64"))] # [cfg (test)] mod tests { use super :: sincosf ; # [test] fn rotational_symmetry () { use core :: f32 :: consts :: PI ; const N : usize = 24 ; for n in 0 .. N { let theta = 2. * PI * (n as f32) / (N as f32) ; let (s , c) = sincosf (theta) ; let (s_plus , c_plus) = sincosf (theta + 2. * PI) ; let (s_minus , c_minus) = sincosf (theta - 2. * PI) ; const TOLERANCE : f32 = 1e-6 ; assert ! ((s - s_plus) . abs () < TOLERANCE , "|{} - {}| = {} >= {}" , s , s_plus , (s - s_plus) . abs () , TOLERANCE) ; assert ! ((s - s_minus) . abs () < TOLERANCE , "|{} - {}| = {} >= {}" , s , s_minus , (s - s_minus) . abs () , TOLERANCE) ; assert ! ((c - c_plus) . abs () < TOLERANCE , "|{} - {}| = {} >= {}" , c , c_plus , (c - c_plus) . abs () , TOLERANCE) ; assert ! ((c - c_minus) . abs () < TOLERANCE , "|{} - {}| = {} >= {}" , c , c_minus , (c - c_minus) . abs () , TOLERANCE) ; } } }
};
}
