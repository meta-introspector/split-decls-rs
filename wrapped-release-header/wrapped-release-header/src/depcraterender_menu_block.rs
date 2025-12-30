// Generated macro for render_menu_block (function)
macro_rules! Depcraterender_menu_block {
() => {
// Module: crate
// Provides: {"render_menu_block"}
// Dependencies: {}
fn render_menu_block (frame : & mut Frame , area : Rect , title : & str , menu_items : & [& str]) { let menu_block = Block :: bordered () . border_type (BorderType :: Rounded) . border_style (MENU_BORDER_COLOR) . padding (Padding :: horizontal (1)) . merge_borders (MergeStrategy :: Fuzzy) . title (title) ; let menu_lines : Vec < Line > = menu_items . iter () . map (| & item | Line :: from (item)) . collect () ; frame . render_widget (Paragraph :: new (menu_lines) . block (menu_block) , area) ; }
};
}
