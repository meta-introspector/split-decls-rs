// Generated macro for render_record (function)
macro_rules! Depcrate_std_shapes_renderrender_record {
() => {
// Module: crate::std_shapes::render
// Provides: {"render_record"}
// Dependencies: {}
fn render_record (rec : & RecordDef , dir : Orientation , loc : Point , size : Point , look : & StyleAttr , canvas : & mut dyn RenderBackend ,) { struct Renderer < 'a > { look : StyleAttr , clip_handle : Option < ClipHandle > , canvas : & 'a mut dyn RenderBackend , } let mut clip_handle : Option < ClipHandle > = Option :: None ; if look . rounded > 0 { let xy = Point :: new (loc . x - size . x / 2. , loc . y - size . y / 2.) ; let ch = canvas . create_clip (xy , size , 15) ; clip_handle = Option :: Some (ch) ; } impl < 'a > RecordVisitor for Renderer < 'a > { fn handle_box (& mut self , loc : Point , size : Point) { self . canvas . draw_rect (Point :: new (loc . x - size . x / 2. , loc . y - size . y / 2.) , Point :: new (size . x , size . y) , & self . look , Option :: None , self . clip_handle ,) ; } fn handle_text (& mut self , loc : Point , _size : Point , label : & str , _port : & Option < String > ,) { self . canvas . draw_text (loc , label , & self . look) ; } } let mut visitor = Renderer { look : look . clone () , clip_handle , canvas , } ; visitor . look . rounded = 0 ; visit_record (rec , dir , loc , size , look , & mut visitor) ; let mut look = look . clone () ; look . fill_color = Option :: None ; canvas . draw_rect (Point :: new (loc . x - size . x / 2. , loc . y - size . y / 2.) , Point :: new (size . x , size . y) , & look , Option :: None , Option :: None ,) ; }
};
}
