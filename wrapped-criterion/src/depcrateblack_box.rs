// Generated macro for black_box (function)
macro_rules! Depcrateblack_box {
() => {
// Module: crate
// Provides: {"black_box"}
// Dependencies: {}
# [doc = " A function that is opaque to the optimizer, used to prevent the compiler from"] # [doc = " optimizing away computations in a benchmark."] # [deprecated (note = "use `std::hint::black_box()` instead")] pub fn black_box < T > (dummy : T) -> T { std :: hint :: black_box (dummy) }
};
}
