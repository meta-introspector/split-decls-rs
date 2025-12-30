// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl InnerIvInit for RabbitCore { fn inner_iv_init (inner : RabbitKeyOnlyCore , iv : & Iv) -> Self { let mut state = inner . state ; state . setup_iv ((* iv) . into ()) ; Self { state } } }
};
}
