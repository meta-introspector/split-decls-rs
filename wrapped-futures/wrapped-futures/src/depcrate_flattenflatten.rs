// Generated macro for Flatten (struct)
macro_rules! Depcrate_flattenFlatten {
() => {
// Module: crate::flatten
// Provides: {"Flatten"}
// Dependencies: {}
# [doc = " Future for the `flatten` combinator, flattening a future-of-a-future to get just"] # [doc = " the result of the final future."] # [doc = ""] # [doc = " This is created by this `Future::flatten` method."] pub struct Flatten < A > where A : Future , A :: Item : IntoFuture { state : Chain < A , < A :: Item as IntoFuture > :: Future , () > , }
};
}
