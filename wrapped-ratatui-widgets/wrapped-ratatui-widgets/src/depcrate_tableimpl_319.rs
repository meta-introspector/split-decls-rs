// Generated macro for impl_319 (impl)
macro_rules! Depcrate_tableimpl_319 {
() => {
// Module: crate::table
// Provides: {"impl_319"}
// Dependencies: {}
impl StatefulWidget for & Table < '_ > { type State = TableState ; fn render (self , area : Rect , buf : & mut Buffer , state : & mut Self :: State) { buf . set_style (area , self . style) ; self . block . as_ref () . render (area , buf) ; let table_area = self . block . inner_if_some (area) ; if table_area . is_empty () { return ; } if state . selected . is_some_and (| s | s >= self . rows . len ()) { state . select (Some (self . rows . len () . saturating_sub (1))) ; } if self . rows . is_empty () { state . select (None) ; } let column_count = self . column_count () ; if state . selected_column . is_some_and (| s | s >= column_count) { state . select_column (Some (column_count . saturating_sub (1))) ; } if column_count == 0 { state . select_column (None) ; } let selection_width = self . selection_width (state) ; let column_widths = self . get_column_widths (table_area . width , selection_width , column_count) ; let (header_area , rows_area , footer_area) = self . layout (table_area) ; self . render_header (header_area , buf , & column_widths) ; self . render_rows (rows_area , buf , state , selection_width , & column_widths) ; self . render_footer (footer_area , buf , & column_widths) ; } }
};
}
