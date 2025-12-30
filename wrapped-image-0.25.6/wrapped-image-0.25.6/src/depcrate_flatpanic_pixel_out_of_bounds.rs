// Generated macro for panic_pixel_out_of_bounds (function)
macro_rules! Depcrate_flatpanic_pixel_out_of_bounds {
() => {
// Module: crate::flat
// Provides: {"panic_pixel_out_of_bounds"}
// Dependencies: {}
# [inline (never)] # [cold] fn panic_pixel_out_of_bounds ((x , y) : (u32 , u32) , bounds : (u32 , u32)) -> ! { panic ! ("Image index {:?} out of bounds {:?}" , (x , y) , bounds) }
};
}
