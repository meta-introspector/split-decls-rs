// Generated macro for render_menu (function)
macro_rules! Depcraterender_menu {
() => {
// Module: crate
// Provides: {"render_menu"}
// Dependencies: {}
fn render_menu (frame : & mut Frame , area : Rect) { let layout = Layout :: vertical (Constraint :: from_lengths ([MAIN_DISHES . len () as u16 + 2 , BACKENDS . len () as u16 + 2 ,])) . spacing (Spacing :: Overlap (1)) ; let [main_dishes_area , backends_area] = area . layout (& layout) ; render_menu_block (frame , main_dishes_area , "Main Courses" , & MAIN_DISHES) ; render_menu_block (frame , backends_area , "Pairings" , & BACKENDS) ; }
};
}
