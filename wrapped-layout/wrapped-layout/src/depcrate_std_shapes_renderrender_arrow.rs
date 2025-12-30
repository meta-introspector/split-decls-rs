// Generated macro for render_arrow (function)
macro_rules! Depcrate_std_shapes_renderrender_arrow {
() => {
// Module: crate::std_shapes::render
// Provides: {"render_arrow"}
// Dependencies: {}
pub fn render_arrow (canvas : & mut dyn RenderBackend , debug : bool , elements : & [Element] , arrow : & Arrow ,) { let path = generate_curve_for_elements (elements , arrow , 30.) ; if debug { for seg in & path { canvas . draw_line (seg . 0 , seg . 1 , & StyleAttr :: debug2 () , Option :: None) ; canvas . draw_circle (seg . 0 , Point :: new (6. , 6.) , & StyleAttr :: debug1 () , Option :: None ,) ; canvas . draw_circle (seg . 1 , Point :: new (6. , 6.) , & StyleAttr :: debug1 () , Option :: None ,) ; } } let dash = match arrow . line_style { LineStyleKind :: None => { return ; } LineStyleKind :: Normal => false , LineStyleKind :: Dashed => true , LineStyleKind :: Dotted => true , } ; let start = matches ! (arrow . start , LineEndKind :: Arrow) ; let end = matches ! (arrow . end , LineEndKind :: Arrow) ; canvas . draw_arrow (& path , dash , (start , end) , & arrow . look , arrow . properties . clone () , & arrow . text ,) ; }
};
}
