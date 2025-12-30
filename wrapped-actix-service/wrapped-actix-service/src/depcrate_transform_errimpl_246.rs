// Generated macro for impl_246 (impl)
macro_rules! Depcrate_transform_errimpl_246 {
() => {
// Module: crate::transform_err
// Provides: {"impl_246"}
// Dependencies: {}
impl < T , S , F , E , Req > TransformMapInitErr < T , S , Req , F , E > { pub (crate) fn new (t : T , f : F) -> Self where T : Transform < S , Req > , F : Fn (T :: InitError) -> E , { Self { transform : t , mapper : f , _phantom : PhantomData , } } }
};
}
