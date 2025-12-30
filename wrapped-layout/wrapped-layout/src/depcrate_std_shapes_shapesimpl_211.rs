// Generated macro for impl_211 (impl)
macro_rules! Depcrate_std_shapes_shapesimpl_211 {
() => {
// Module: crate::std_shapes::shapes
// Provides: {"impl_211"}
// Dependencies: {}
impl Visible for Element { fn position (& self) -> Position { self . pos } fn position_mut (& mut self) -> & mut Position { & mut self . pos } fn is_connector (& self) -> bool { matches ! (self . shape , ShapeKind :: Connector (_)) } fn transpose (& mut self) { self . orientation = self . orientation . flip () ; self . pos . transpose () ; } fn resize (& mut self) { if let ShapeKind :: Connector (_) = self . shape . clone () { let size = get_shape_size (self . orientation , & self . shape , self . look . font_size , false ,) ; self . pos . set_size (size) ; match self . orientation { Orientation :: TopToBottom => { self . pos . set_new_center_point (Point :: new (0. , size . y / 2.)) ; } Orientation :: LeftToRight => { self . pos . set_new_center_point (Point :: new (size . x / 2. , 0.)) ; } } } } }
};
}
