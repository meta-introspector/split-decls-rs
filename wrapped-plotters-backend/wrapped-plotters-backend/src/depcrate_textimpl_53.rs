// Generated macro for impl_53 (impl)
macro_rules! Depcrate_textimpl_53 {
() => {
// Module: crate::text
// Provides: {"impl_53"}
// Dependencies: {}
impl FontTransform { # [doc = " Transform the coordinate to perform the rotation"] # [doc = ""] # [doc = " - `x`: The x coordinate in pixels before transform"] # [doc = " - `y`: The y coordinate in pixels before transform"] # [doc = " - **returns**: The coordinate after transform"] pub fn transform (& self , x : i32 , y : i32) -> (i32 , i32) { match self { FontTransform :: None => (x , y) , FontTransform :: Rotate90 => (- y , x) , FontTransform :: Rotate180 => (- x , - y) , FontTransform :: Rotate270 => (y , - x) , } } }
};
}
