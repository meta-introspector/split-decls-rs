// Generated macro for Trial (struct)
macro_rules! DepcrateTrial {
() => {
// Module: crate
// Provides: {"Trial"}
// Dependencies: {}
# [doc = " A single test or benchmark."] # [doc = ""] # [doc = " The original `libtest` often calls benchmarks \"tests\", which is a bit"] # [doc = " confusing. So in this library, it is called \"trial\"."] # [doc = ""] # [doc = " A trial is created via [`Trial::test`] or [`Trial::bench`]. The trial's"] # [doc = " `name` is printed and used for filtering. The `runner` is called when the"] # [doc = " test/benchmark is executed to determine its outcome. If `runner` panics,"] # [doc = " the trial is considered \"failed\". If you need the behavior of"] # [doc = " `#[should_panic]` you need to catch the panic yourself. You likely want to"] # [doc = " compare the panic payload to an expected value anyway."] pub struct Trial { runner : Box < dyn FnOnce (bool) -> Outcome + Send > , info : TestInfo , }
};
}
