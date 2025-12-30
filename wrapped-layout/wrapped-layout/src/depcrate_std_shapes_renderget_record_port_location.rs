// Generated macro for get_record_port_location (function)
macro_rules! Depcrate_std_shapes_renderget_record_port_location {
() => {
// Module: crate::std_shapes::render
// Provides: {"get_record_port_location"}
// Dependencies: {}
fn get_record_port_location (rec : & RecordDef , dir : Orientation , loc : Point , size : Point , look : & StyleAttr , port_name : & str ,) -> (Point , Point) { struct Locator { port_name : String , loc : Point , size : Point , } impl RecordVisitor for Locator { fn handle_box (& mut self , _loc : Point , _size : Point) { } fn handle_text (& mut self , loc : Point , size : Point , _label : & str , port : & Option < String > ,) { if let Option :: Some (port_name) = port { if * port_name == self . port_name { self . loc = loc ; self . size = size ; } } } } let mut visitor = Locator { port_name : port_name . to_string () , loc , size , } ; visit_record (rec , dir , loc , size , look , & mut visitor) ; (visitor . loc , visitor . size) }
};
}
