// Generated macro for check_color (function)
macro_rules! Depcrate_drawing_backend_impl_mockedcheck_color {
() => {
// Module: crate::drawing::backend_impl::mocked
// Provides: {"check_color"}
// Dependencies: {}
pub fn check_color (left : BackendColor , right : RGBAColor) { assert_eq ! (RGBAColor (left . rgb . 0 , left . rgb . 1 , left . rgb . 2 , left . alpha) , right) ; }
};
}
