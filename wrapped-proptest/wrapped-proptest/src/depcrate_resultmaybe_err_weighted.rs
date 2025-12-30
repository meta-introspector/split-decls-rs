// Generated macro for maybe_err_weighted (function)
macro_rules! Depcrate_resultmaybe_err_weighted {
() => {
// Module: crate::result
// Provides: {"maybe_err_weighted"}
// Dependencies: {}
# [doc = " Create a strategy for `Result`s where `Ok` values are taken from `t` and"] # [doc = " `Err` values are taken from `e`."] # [doc = ""] # [doc = " `probability_of_ok` is the probability (between 0.0 and 1.0, exclusive)"] # [doc = " that `Err` is initially chosen."] # [doc = ""] # [doc = " Generated values shrink to `Ok`."] pub fn maybe_err_weighted < T : Strategy , E : Strategy > (probability_of_err : impl Into < Probability > , t : T , e : E ,) -> MaybeErr < T , E > { let prob = probability_of_err . into () . into () ; let (err_weight , ok_weight) = float_to_weight (prob) ; MaybeErr (TupleUnion :: new (((ok_weight , Arc :: new (statics :: Map :: new (t , WrapOk (PhantomData , PhantomData))) ,) , (err_weight , Arc :: new (statics :: Map :: new (e , WrapErr (PhantomData , PhantomData))) ,) ,))) }
};
}
