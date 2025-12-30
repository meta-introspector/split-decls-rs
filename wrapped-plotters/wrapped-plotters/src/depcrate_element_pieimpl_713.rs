// Generated macro for impl_713 (impl)
macro_rules! Depcrate_element_pieimpl_713 {
() => {
// Module: crate::element::pie
// Provides: {"impl_713"}
// Dependencies: {}
impl < 'a , Label : Display > PointCollection < 'a , (i32 , i32) > for & 'a Pie < 'a , (i32 , i32) , Label > { type Point = & 'a (i32 , i32) ; type IntoIter = std :: iter :: Once < & 'a (i32 , i32) > ; fn point_iter (self) -> std :: iter :: Once < & 'a (i32 , i32) > { std :: iter :: once (self . center) } }
};
}
