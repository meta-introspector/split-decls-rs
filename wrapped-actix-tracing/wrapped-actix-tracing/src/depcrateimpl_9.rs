// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < S , U , F > TracingTransform < S , U , F > { # [doc = " Constructs new tracing middleware."] pub fn new (make_span : F) -> Self { TracingTransform { make_span , _p : PhantomData , } } }
};
}
