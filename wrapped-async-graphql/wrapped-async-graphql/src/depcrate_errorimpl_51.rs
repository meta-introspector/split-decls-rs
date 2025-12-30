// Generated macro for impl_51 (impl)
macro_rules! Depcrate_errorimpl_51 {
() => {
// Module: crate::error
// Provides: {"impl_51"}
// Dependencies: {}
impl < T : InputType , E : Display > From < E > for InputValueError < T > { fn from (error : E) -> Self { Self :: custom (error) } }
};
}
