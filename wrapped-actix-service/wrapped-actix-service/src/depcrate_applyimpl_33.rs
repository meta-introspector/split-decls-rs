// Generated macro for impl_33 (impl)
macro_rules! Depcrate_applyimpl_33 {
() => {
// Module: crate::apply
// Provides: {"impl_33"}
// Dependencies: {}
impl < S , F , Fut , Req , In , Res , Err > Apply < S , F , Req , In , Res , Err > where S : Service < In , Error = Err > , F : Fn (Req , & S) -> Fut , Fut : Future < Output = Result < Res , Err > > , { # [doc = " Create new `Apply` combinator"] fn new (service : S , wrap_fn : F) -> Self { Self { service , wrap_fn , _phantom : PhantomData , } } }
};
}
