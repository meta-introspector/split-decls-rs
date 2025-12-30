// Generated macro for impl_581 (impl)
macro_rules! Depcrate_element_textimpl_581 {
() => {
// Module: crate::element::text
// Provides: {"impl_581"}
// Dependencies: {}
impl < 'b , 'a , Coord : 'a , T : Borrow < str > + 'a > PointCollection < 'a , Coord > for & 'a MultiLineText < 'b , Coord , T > { type Point = & 'a Coord ; type IntoIter = std :: iter :: Once < & 'a Coord > ; fn point_iter (self) -> Self :: IntoIter { std :: iter :: once (& self . coord) } }
};
}
