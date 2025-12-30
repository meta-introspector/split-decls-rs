// Generated macro for final_func (function)
macro_rules! Depcrate_helpersfinal_func {
() => {
// Module: crate::helpers
// Provides: {"final_func"}
// Dependencies: {}
pub (crate) fn final_func < F > (mut index : i32 , condition : F) -> i32 where F : Fn (i32) -> bool , { while condition (index) { index += 1 ; } index - 1 }
};
}
