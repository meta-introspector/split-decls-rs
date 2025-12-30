// Generated macro for ApplyFactory (struct)
macro_rules! Depcrate_applyApplyFactory {
() => {
// Module: crate::apply
// Provides: {"ApplyFactory"}
// Dependencies: {}
# [doc = " `ApplyFactory` service factory combinator."] pub struct ApplyFactory < SF , F , Req , In , Res , Err > { factory : SF , wrap_fn : F , _phantom : PhantomData < fn (Req) -> (In , Res , Err) > , }
};
}
