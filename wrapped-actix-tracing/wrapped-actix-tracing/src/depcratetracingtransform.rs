// Generated macro for TracingTransform (struct)
macro_rules! DepcrateTracingTransform {
() => {
// Module: crate
// Provides: {"TracingTransform"}
// Dependencies: {}
# [doc = " A `Transform` implementation that wraps services with a [`TracingService`]."] pub struct TracingTransform < S , U , F > { make_span : F , _p : PhantomData < fn (S , U) > , }
};
}
