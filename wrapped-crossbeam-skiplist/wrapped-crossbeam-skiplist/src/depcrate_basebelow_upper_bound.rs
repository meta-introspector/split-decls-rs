// Generated macro for below_upper_bound (function)
macro_rules! Depcrate_basebelow_upper_bound {
() => {
// Module: crate::base
// Provides: {"below_upper_bound"}
// Dependencies: {}
# [doc = " Helper function to check if a value is below an upper bound"] fn below_upper_bound < V , T > (bound : & Bound < & T > , other : & V) -> bool where T : ? Sized , V : Comparable < T > , { match * bound { Bound :: Unbounded => true , Bound :: Included (key) => other . compare (key) . is_le () , Bound :: Excluded (key) => other . compare (key) . is_lt () , } }
};
}
