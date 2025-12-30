// Generated macro for macro_1236 (macro)
macro_rules! Depcrate_collection_is_never_readmacro_1236 {
() => {
// Module: crate::collection_is_never_read
// Provides: {"macro_1236"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for collections that are never queried."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Putting effort into constructing a collection but then never querying it might indicate that"] # [doc = " the author forgot to do whatever they intended to do with the collection. Example: Clone"] # [doc = " a vector, sort it for iteration, but then mistakenly iterate the original vector"] # [doc = " instead."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let samples = vec![3, 1, 2];"] # [doc = " let mut sorted_samples = samples.clone();"] # [doc = " sorted_samples.sort();"] # [doc = " for sample in &samples { // Oops, meant to use `sorted_samples`."] # [doc = "     println!(\"{sample}\");"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let samples = vec![3, 1, 2];"] # [doc = " let mut sorted_samples = samples.clone();"] # [doc = " sorted_samples.sort();"] # [doc = " for sample in &sorted_samples {"] # [doc = "     println!(\"{sample}\");"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub COLLECTION_IS_NEVER_READ , nursery , "a collection is never queried" }
};
}
