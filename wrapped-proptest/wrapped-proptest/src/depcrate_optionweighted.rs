// Generated macro for weighted (function)
macro_rules! Depcrate_optionweighted {
() => {
// Module: crate::option
// Provides: {"weighted"}
// Dependencies: {}
# [doc = " Return a strategy producing `Optional` values wrapping values from the"] # [doc = " given delegate strategy."] # [doc = ""] # [doc = " `Some` values shrink to `None`."] # [doc = ""] # [doc = " `Some` is chosen with a probability given by `probability_of_some`, which"] # [doc = " must be between 0.0 and 1.0, both exclusive."] pub fn weighted < T : Strategy > (probability_of_some : impl Into < Probability > , t : T ,) -> OptionStrategy < T > { let prob = probability_of_some . into () . into () ; let (weight_some , weight_none) = float_to_weight (prob) ; OptionStrategy (TupleUnion :: new (((weight_none , Arc :: new (NoneStrategy (PhantomData))) , (weight_some , Arc :: new (statics :: Map :: new (t , WrapSome))) ,))) }
};
}
