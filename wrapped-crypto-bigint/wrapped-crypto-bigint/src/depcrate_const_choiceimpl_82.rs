// Generated macro for impl_82 (impl)
macro_rules! Depcrate_const_choiceimpl_82 {
() => {
// Module: crate::const_choice
// Provides: {"impl_82"}
// Dependencies: {}
impl < T > From < ConstCtOption < T > > for CtOption < T > { # [inline] fn from (value : ConstCtOption < T >) -> Self { CtOption :: new (value . value , value . is_some . into ()) } }
};
}
