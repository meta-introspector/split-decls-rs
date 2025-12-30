// Generated macro for UselessVec (struct)
macro_rules! Depcrate_useless_vecUselessVec {
() => {
// Module: crate::useless_vec
// Provides: {"UselessVec"}
// Dependencies: {}
pub struct UselessVec { too_large_for_stack : u64 , msrv : Msrv , # [doc = " Maps from a `vec![]` source callsite invocation span to the \"state\" (i.e., whether we can"] # [doc = " emit a warning there or not)."] # [doc = ""] # [doc = " The purpose of this is to buffer lints up until `check_crate_post` so that we can cancel a"] # [doc = " lint while visiting, because a `vec![]` invocation span can appear multiple times when"] # [doc = " it is passed as a macro argument, once in a context that doesn't require a `Vec<_>` and"] # [doc = " another time that does. Consider:"] # [doc = " ```"] # [doc = " macro_rules! m {"] # [doc = "     ($v:expr) => {"] # [doc = "         let a = $v;"] # [doc = "         $v.push(3);"] # [doc = "     }"] # [doc = " }"] # [doc = " m!(vec![1, 2]);"] # [doc = " ```"] # [doc = " The macro invocation expands to two `vec![1, 2]` invocations. If we eagerly suggest changing"] # [doc = " the first `vec![1, 2]` (which is shared with the other expn) to an array which indeed would"] # [doc = " work, we get a false positive warning on the `$v.push(3)` which really requires `$v` to"] # [doc = " be a vector."] span_to_state : BTreeMap < Span , VecState > , allow_in_test : bool , }
};
}
