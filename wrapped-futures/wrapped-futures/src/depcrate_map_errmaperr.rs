// Generated macro for MapErr (struct)
macro_rules! Depcrate_map_errMapErr {
() => {
// Module: crate::map_err
// Provides: {"MapErr"}
// Dependencies: {}
# [doc = " Future for the `map_err` combinator, changing the error type of a future."] # [doc = ""] # [doc = " This is created by this `Future::map_err` method."] pub struct MapErr < A , F > where A : Future { future : Collapsed < A > , f : Option < F > , }
};
}
