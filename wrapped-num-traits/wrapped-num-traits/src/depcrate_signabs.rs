// Generated macro for abs (function)
macro_rules! Depcrate_signabs {
() => {
// Module: crate::sign
// Provides: {"abs"}
// Dependencies: {}
# [doc = " Computes the absolute value."] # [doc = ""] # [doc = " For `f32` and `f64`, `NaN` will be returned if the number is `NaN`"] # [doc = ""] # [doc = " For signed integers, `::MIN` will be returned if the number is `::MIN`."] # [inline (always)] pub fn abs < T : Signed > (value : T) -> T { value . abs () }
};
}
