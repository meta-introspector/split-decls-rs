// Generated macro for dot (function)
macro_rules! Depcrate_statsdot {
() => {
// Module: crate::stats
// Provides: {"dot"}
// Dependencies: {}
fn dot < A > (xs : & [A] , ys : & [A]) -> A where A : Float , { xs . iter () . zip (ys) . fold (A :: cast (0) , | acc , (& x , & y) | acc + x * y) }
};
}
