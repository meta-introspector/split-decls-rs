// Generated macro for fill_row (function)
macro_rules! Depcrate_format_textfill_row {
() => {
// Module: crate::format::text
// Provides: {"fill_row"}
// Dependencies: {}
# [doc = " Formats a row, filling cells with spaces and word-wrapping text."] # [doc = ""] # [doc = " Returns a vec of cells, where each cell is split into multiple lines."] fn fill_row (row : & [String] , col_widths : & [usize] , alignment : & [Alignment]) -> Vec < Vec < String > > { let mut cell_lines = row . iter () . zip (col_widths) . zip (alignment) . map (| ((cell , width) , alignment) | fill_cell (cell , * width - 2 , * alignment)) . collect :: < Vec < _ > > () ; let max_lines = cell_lines . iter () . map (| cell | cell . len ()) . max () . unwrap () ; for (cell , width) in cell_lines . iter_mut () . zip (col_widths) { if cell . len () < max_lines { cell . extend (std :: iter :: repeat (" " . repeat (* width)) . take (max_lines - cell . len ())) ; } } cell_lines }
};
}
