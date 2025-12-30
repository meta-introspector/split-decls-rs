// Generated macro for impl_207 (impl)
macro_rules! Depcrate_std_shapes_shapesimpl_207 {
() => {
// Module: crate::std_shapes::shapes
// Provides: {"impl_207"}
// Dependencies: {}
impl Element { pub fn create (shape : ShapeKind , look : StyleAttr , orientation : Orientation , size : Point ,) -> Element { Element { shape , look , orientation , pos : Position :: new (Point :: zero () , size , Point :: zero () , Point :: splat (PADDING) ,) , properties : Option :: None , } } pub fn create_with_properties (shape : ShapeKind , look : StyleAttr , orientation : Orientation , size : Point , properties : impl Into < String > ,) -> Element { let mut elem = Element :: create (shape , look , orientation , size) ; elem . properties = Option :: Some (properties . into ()) ; elem } pub fn create_connector (label : & str , look : & StyleAttr , dir : Orientation ,) -> Element { Element { shape : ShapeKind :: new_connector (label) , look : look . clone () , orientation : dir , pos : Position :: new (Point :: zero () , Point :: zero () , Point :: zero () , Point :: splat (CONN_PADDING) ,) , properties : Option :: None , } } pub fn empty_connector (dir : Orientation) -> Element { Self :: create_connector ("" , & StyleAttr :: simple () , dir) } pub fn move_to (& mut self , to : Point) { self . pos . move_to (to) } }
};
}
