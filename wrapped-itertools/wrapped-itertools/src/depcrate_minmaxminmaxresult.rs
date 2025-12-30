// Generated macro for MinMaxResult (enum)
macro_rules! Depcrate_minmaxMinMaxResult {
() => {
// Module: crate::minmax
// Provides: {"MinMaxResult"}
// Dependencies: {}
# [doc = " `MinMaxResult` is an enum returned by `minmax`."] # [doc = ""] # [doc = " See [`.minmax()`](crate::Itertools::minmax) for more detail."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum MinMaxResult < T > { # [doc = " Empty iterator"] NoElements , # [doc = " Iterator with one element, so the minimum and maximum are the same"] OneElement (T) , # [doc = " More than one element in the iterator, the first element is not larger"] # [doc = " than the second"] MinMax (T , T) , }
};
}
