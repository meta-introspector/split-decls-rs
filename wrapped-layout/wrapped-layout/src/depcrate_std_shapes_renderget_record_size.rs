// Generated macro for get_record_size (function)
macro_rules! Depcrate_std_shapes_renderget_record_size {
() => {
// Module: crate::std_shapes::render
// Provides: {"get_record_size"}
// Dependencies: {}
# [doc = " Return the height and width of the record, depending on the geometry and"] # [doc = " internal text."] fn get_record_size (rec : & RecordDef , dir : Orientation , font_size : usize ,) -> Point { match rec { RecordDef :: Text (label , _) => pad_shape_scalar (get_size_for_str (label , font_size) , BOX_SHAPE_PADDING ,) , RecordDef :: Array (arr) => { let mut x : f64 = 0. ; let mut y : f64 = 0. ; for elem in arr { let ret = get_record_size (elem , dir . flip () , font_size) ; if dir . is_left_right () { x += ret . x ; y = y . max (ret . y) ; } else { x = x . max (ret . x) ; y += ret . y ; } } Point :: new (x , y) } } }
};
}
