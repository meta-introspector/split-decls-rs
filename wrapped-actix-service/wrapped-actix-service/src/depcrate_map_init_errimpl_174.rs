// Generated macro for impl_174 (impl)
macro_rules! Depcrate_map_init_errimpl_174 {
() => {
// Module: crate::map_init_err
// Provides: {"impl_174"}
// Dependencies: {}
impl < A , F , Req , Err > MapInitErr < A , F , Req , Err > where A : ServiceFactory < Req > , F : Fn (A :: InitError) -> Err , { # [doc = " Create new `MapInitErr` combinator"] pub (crate) fn new (a : A , f : F) -> Self { Self { a , f , e : PhantomData , } } }
};
}
