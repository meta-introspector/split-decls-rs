// Generated macro for impl_87 (impl)
macro_rules! Depcrate_core_styleimpl_87 {
() => {
// Module: crate::core::style
// Provides: {"impl_87"}
// Dependencies: {}
impl StyleAttr { pub fn new (line_color : Color , line_width : usize , fill_color : Option < Color > , rounded : usize , font_size : usize ,) -> Self { Self { line_color , line_width , fill_color , rounded , font_size , } } pub fn simple () -> Self { StyleAttr :: new (Color :: fast ("black") , 2 , Option :: Some (Color :: fast ("white")) , 0 , 15 ,) } pub fn debug0 () -> Self { StyleAttr :: new (Color :: fast ("black") , 1 , Option :: Some (Color :: fast ("pink")) , 0 , 15 ,) } pub fn debug1 () -> Self { StyleAttr :: new (Color :: fast ("black") , 1 , Option :: Some (Color :: fast ("aliceblue")) , 0 , 15 ,) } pub fn debug2 () -> Self { StyleAttr :: new (Color :: fast ("black") , 1 , Option :: Some (Color :: fast ("white")) , 0 , 15 ,) } }
};
}
