// Generated macro for maybe_ok_weighted (function)
macro_rules! Depcrate_resultmaybe_ok_weighted {
() => {
// Module: crate::result
// Provides: {"maybe_ok_weighted"}
// Dependencies: {}
# [doc = " Create a strategy for `Result`s where `Ok` values are taken from `t` and"] # [doc = " `Err` values are taken from `e`."] # [doc = ""] # [doc = " `probability_of_ok` is the probability (between 0.0 and 1.0, exclusive)"] # [doc = " that `Ok` is initially chosen."] # [doc = ""] # [doc = " Generated values shrink to `Err`."] pub fn maybe_ok_weighted < T : Strategy , E : Strategy > (probability_of_ok : impl Into < Probability > , t : T , e : E ,) -> MaybeOk < T , E > { let prob = probability_of_ok . into () . into () ; let (ok_weight , err_weight) = float_to_weight (prob) ; MaybeOk (TupleUnion :: new (((err_weight , Arc :: new (statics :: Map :: new (e , WrapErr (PhantomData , PhantomData))) ,) , (ok_weight , Arc :: new (statics :: Map :: new (t , WrapOk (PhantomData , PhantomData))) ,) ,))) }
};
}
