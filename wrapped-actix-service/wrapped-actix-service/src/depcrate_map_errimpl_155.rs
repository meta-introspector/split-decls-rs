// Generated macro for impl_155 (impl)
macro_rules! Depcrate_map_errimpl_155 {
() => {
// Module: crate::map_err
// Provides: {"impl_155"}
// Dependencies: {}
impl < S , Req , F , E > MapErr < S , Req , F , E > { # [doc = " Create new `MapErr` combinator"] pub (crate) fn new (service : S , mapper : F) -> Self where S : Service < Req > , F : Fn (S :: Error) -> E , { Self { service , mapper , _t : PhantomData , } } }
};
}
