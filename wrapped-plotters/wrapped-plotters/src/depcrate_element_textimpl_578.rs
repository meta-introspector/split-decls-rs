// Generated macro for impl_578 (impl)
macro_rules! Depcrate_element_textimpl_578 {
() => {
// Module: crate::element::text
// Provides: {"impl_578"}
// Dependencies: {}
impl < 'a , T : Borrow < str > > MultiLineText < 'a , BackendCoord , T > { # [doc = " Compute the line layout"] pub fn compute_line_layout (& self) -> FontResult < Vec < LayoutBox > > { let mut ret = vec ! [] ; for ((x , y) , t) in self . layout_lines (self . coord) . zip (self . lines . iter ()) { let (dx , dy) = self . style . font . box_size (t . borrow ()) ? ; ret . push (((x , y) , (x + dx as i32 , y + dy as i32))) ; } Ok (ret) } }
};
}
