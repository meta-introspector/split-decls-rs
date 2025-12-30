// Generated macro for above_lower_bound (function)
macro_rules! Depcrate_baseabove_lower_bound {
() => {
// Module: crate::base
// Provides: {"above_lower_bound"}
// Dependencies: {}
# [doc = " Helper function to check if a value is above a lower bound"] fn above_lower_bound < V , T > (bound : & Bound < & T > , other : & V) -> bool where T : ? Sized , V : Comparable < T > , { match * bound { Bound :: Unbounded => true , Bound :: Included (key) => other . compare (key) . is_ge () , Bound :: Excluded (key) => other . compare (key) . is_gt () , } }
};
}
