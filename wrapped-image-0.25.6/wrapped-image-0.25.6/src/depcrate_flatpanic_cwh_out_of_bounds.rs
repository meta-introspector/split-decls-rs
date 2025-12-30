// Generated macro for panic_cwh_out_of_bounds (function)
macro_rules! Depcrate_flatpanic_cwh_out_of_bounds {
() => {
// Module: crate::flat
// Provides: {"panic_cwh_out_of_bounds"}
// Dependencies: {}
# [inline (never)] # [cold] fn panic_cwh_out_of_bounds ((c , x , y) : (u8 , u32 , u32) , bounds : (u8 , u32 , u32) , strides : (usize , usize , usize) ,) -> ! { panic ! ("Sample coordinates {:?} out of sample matrix bounds {:?} with strides {:?}" , (c , x , y) , bounds , strides) }
};
}
