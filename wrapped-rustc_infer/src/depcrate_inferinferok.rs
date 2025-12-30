// Generated macro for InferOk (struct)
macro_rules! Depcrate_inferInferOk {
() => {
// Module: crate::infer
// Provides: {"InferOk"}
// Dependencies: {}
# [doc = " `InferOk<'tcx, ()>` is used a lot. It may seem like a useless wrapper"] # [doc = " around `PredicateObligations<'tcx>`, but it has one important property:"] # [doc = " because `InferOk` is marked with `#[must_use]`, if you have a method"] # [doc = " `InferCtxt::f` that returns `InferResult<'tcx, ()>` and you call it with"] # [doc = " `infcx.f()?;` you'll get a warning about the obligations being discarded"] # [doc = " without use, which is probably unintentional and has been a source of bugs"] # [doc = " in the past."] # [must_use] # [derive (Debug)] pub struct InferOk < 'tcx , T > { pub value : T , pub obligations : PredicateObligations < 'tcx > , }
};
}
