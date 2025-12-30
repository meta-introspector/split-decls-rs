// Generated macro for impl_572 (impl)
macro_rules! Depcrate_element_textimpl_572 {
() => {
// Module: crate::element::text
// Provides: {"impl_572"}
// Dependencies: {}
impl < 'b , 'a , Coord : 'a , T : Borrow < str > + 'a > PointCollection < 'a , Coord > for & 'a Text < 'b , Coord , T > { type Point = & 'a Coord ; type IntoIter = std :: iter :: Once < & 'a Coord > ; fn point_iter (self) -> Self :: IntoIter { std :: iter :: once (& self . coord) } }
};
}
