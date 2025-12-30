// Generated macro for impl_501 (impl)
macro_rules! Depcrate_errorimpl_501 {
() => {
// Module: crate::error
// Provides: {"impl_501"}
// Dependencies: {}
impl From < RangeError > for DateError { # [inline] fn from (value : RangeError) -> Self { let RangeError { field , value , min , max , } = value ; DateError :: Range { field , value , min , max , } } }
};
}
