// Generated macro for Map (struct)
macro_rules! Depcrate_mapMap {
() => {
// Module: crate::map
// Provides: {"Map"}
// Dependencies: {}
# [doc = " Future for the `map` combinator, changing the type of a future."] # [doc = ""] # [doc = " This is created by this `Future::map` method."] pub struct Map < A , F > where A : Future { future : Collapsed < A > , f : Option < F > , }
};
}
