// Generated macro for impl_83 (impl)
macro_rules! Depcrate_const_choiceimpl_83 {
() => {
// Module: crate::const_choice
// Provides: {"impl_83"}
// Dependencies: {}
impl < T > From < ConstCtOption < T > > for Option < T > { # [inline] fn from (value : ConstCtOption < T >) -> Self { if value . is_some . into () { Some (value . value) } else { None } } }
};
}
