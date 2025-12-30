// Generated macro for visit_record (function)
macro_rules! Depcrate_std_shapes_rendervisit_record {
() => {
// Module: crate::std_shapes::render
// Provides: {"visit_record"}
// Dependencies: {}
fn visit_record (rec : & RecordDef , dir : Orientation , loc : Point , size : Point , look : & StyleAttr , visitor : & mut dyn RecordVisitor ,) { visitor . handle_box (loc , size) ; match rec { RecordDef :: Text (text , port) => { visitor . handle_text (loc , size , text , port) ; } RecordDef :: Array (arr) => { let mut sizes : Vec < Point > = Vec :: new () ; let mut sum = Point :: zero () ; let mut mx = Point :: zero () ; for elem in arr { let sz = get_record_size (elem , dir , look . font_size) ; sizes . push (sz) ; sum = Point :: new (sum . x + sz . x , sum . y + sz . y) ; mx = Point :: new (mx . x . max (sz . x) , mx . y . max (sz . y)) ; } for sz in & mut sizes { if dir . is_left_right () { * sz = Point :: new (size . x * sz . x / sum . x , size . y) ; } else { * sz = Point :: new (size . x , size . y * sz . y / sum . y) ; } } if dir . is_left_right () { let mut startx = loc . x - size . x / 2. ; for i in 0 .. sizes . len () { let element = & arr [i] ; let loc2 = Point :: new (startx + sizes [i] . x / 2. , loc . y) ; visit_record (element , dir . flip () , loc2 , sizes [i] , look , visitor ,) ; startx += sizes [i] . x ; } } else { let mut starty = loc . y - size . y / 2. ; for i in 0 .. sizes . len () { let element = & arr [i] ; let loc2 = Point :: new (loc . x , starty + sizes [i] . y / 2.) ; visit_record (element , dir . flip () , loc2 , sizes [i] , look , visitor ,) ; starty += sizes [i] . y ; } } } } }
};
}
