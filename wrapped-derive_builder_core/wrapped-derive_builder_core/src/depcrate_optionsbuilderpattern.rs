// Generated macro for BuilderPattern (enum)
macro_rules! Depcrate_optionsBuilderPattern {
() => {
// Module: crate::options
// Provides: {"BuilderPattern"}
// Dependencies: {}
# [doc = " Controls the signature of a setter method,"] # [doc = " more specifically how `self` is passed and returned."] # [doc = ""] # [doc = " It can also be generalized to methods with different parameter sets and"] # [doc = " return types, e.g. the `build()` method."] # [derive (PartialEq , Eq , Debug , Clone , Copy , FromMeta)] pub enum BuilderPattern { # [doc = " E.g. `fn bar(self, bar: Bar) -> Self`."] Owned , # [doc = " E.g. `fn bar(&mut self, bar: Bar) -> &mut Self`."] Mutable , # [doc = " E.g. `fn bar(&self, bar: Bar) -> Self`."] # [doc = ""] # [doc = " Note:"] # [doc = " - Needs to `clone` in order to return an _updated_ instance of `Self`."] # [doc = " - There is a great chance that the Rust compiler (LLVM) will"] # [doc = "   optimize chained `clone` calls away in release mode."] # [doc = "   Therefore this turns out not to be as bad as it sounds."] Immutable , }
};
}
