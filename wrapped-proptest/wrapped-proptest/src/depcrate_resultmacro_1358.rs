// Generated macro for macro_1358 (macro)
macro_rules! Depcrate_resultmacro_1358 {
() => {
// Module: crate::result
// Provides: {"macro_1358"}
// Dependencies: {}
opaque_strategy_wrapper ! { # [doc = " Strategy which generates `Result`s using `Ok` and `Err` values from two"] # [doc = " delegate strategies."] # [doc = ""] # [doc = " Shrinks to `Err`."] # [derive (Clone)] pub struct MaybeOk [< T , E >] [where T : Strategy , E : Strategy] (TupleUnion < (WA < MapErr < T , E >>, WA < MapOk < T , E >>) >) -> MaybeOkValueTree < T , E >; # [doc = " `ValueTree` type corresponding to `MaybeOk`."] pub struct MaybeOkValueTree [< T , E >] [where T : Strategy , E : Strategy] (TupleUnionValueTree < (LazyValueTree < statics :: Map < E , WrapErr < T :: Value , E :: Value >>>, Option < LazyValueTree < statics :: Map < T , WrapOk < T :: Value , E :: Value >>>>,) >) -> Result < T :: Value , E :: Value >; }
};
}
