// Generated macro for macro_1359 (macro)
macro_rules! Depcrate_resultmacro_1359 {
() => {
// Module: crate::result
// Provides: {"macro_1359"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy which generates `Result`s using `Ok` and `Err` values from two"] # [doc = " delegate strategies."] # [doc = ""] # [doc = " Shrinks to `Ok`."] # [derive (Clone)] pub struct MaybeErr [< T , E >] [where T : Strategy , E : Strategy] (TupleUnion < (WA < MapOk < T , E >>, WA < MapErr < T , E >>) >) -> MaybeErrValueTree < T , E >; # [doc = " `ValueTree` type corresponding to `MaybeErr`."] pub struct MaybeErrValueTree [< T , E >] [where T : Strategy , E : Strategy] (TupleUnionValueTree < (LazyValueTree < statics :: Map < T , WrapOk < T :: Value , E :: Value >>>, Option < LazyValueTree < statics :: Map < E , WrapErr < T :: Value , E :: Value >>>>,) >) -> Result < T :: Value , E :: Value >; }
};
}
