// Generated macro for private (module)
macro_rules! Depcrate_range_inclusiveprivate {
() => {
// Module: crate::range_inclusive
// Provides: {"private"}
// Dependencies: {}
# [doc = " These traits help drive integer type inference. Without them, an unknown `{integer}` type only"] # [doc = " has constraints on `Iter<{integer}>`, which will probably give up and use `i32`. By adding"] # [doc = " these traits on the item type, the compiler can see a more direct constraint to infer like"] # [doc = " `{integer}: RangeInteger`, which works better. See `test_issue_833` for an example."] # [doc = ""] # [doc = " They have to be `pub` since they're seen in the public `impl ParallelIterator` constraints, but"] # [doc = " we put them in a private modules so they're not actually reachable in our public API."] mod private { use super :: * ; # [doc = " Implementation details of `ParallelIterator for Iter<Self>`"] pub trait RangeInteger : Sized + Send { private_decl ! { } fn drive_unindexed < C > (iter : Iter < Self > , consumer : C) -> C :: Result where C : UnindexedConsumer < Self > ; fn opt_len (iter : & Iter < Self >) -> Option < usize > ; } # [doc = " Implementation details of `IndexedParallelIterator for Iter<Self>`"] pub trait IndexedRangeInteger : RangeInteger { private_decl ! { } fn drive < C > (iter : Iter < Self > , consumer : C) -> C :: Result where C : Consumer < Self > ; fn len (iter : & Iter < Self >) -> usize ; fn with_producer < CB > (iter : Iter < Self > , callback : CB) -> CB :: Output where CB : ProducerCallback < Self > ; } }
};
}
