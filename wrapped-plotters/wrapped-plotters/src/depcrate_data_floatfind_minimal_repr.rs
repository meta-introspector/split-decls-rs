// Generated macro for find_minimal_repr (function)
macro_rules! Depcrate_data_floatfind_minimal_repr {
() => {
// Module: crate::data::float
// Provides: {"find_minimal_repr"}
// Dependencies: {}
fn find_minimal_repr (n : f64 , eps : f64) -> (f64 , usize) { if eps >= 1.0 { return (n , 0) ; } if n - n . floor () < eps { (n . floor () , 0) } else if n . ceil () - n < eps { (n . ceil () , 0) } else { let (rem , pre) = find_minimal_repr ((n - n . floor ()) * 10.0 , eps * 10.0) ; (n . floor () + rem / 10.0 , pre + 1) } }
};
}
