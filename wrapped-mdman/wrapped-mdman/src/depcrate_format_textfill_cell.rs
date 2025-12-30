// Generated macro for fill_cell (function)
macro_rules! Depcrate_format_textfill_cell {
() => {
// Module: crate::format::text
// Provides: {"fill_cell"}
// Dependencies: {}
# [doc = " Formats a cell. Word-wraps based on width, and adjusts based on alignment."] # [doc = ""] # [doc = " Returns a vec of lines for the cell."] fn fill_cell (text : & str , width : usize , alignment : Alignment) -> Vec < String > { let fill_width = | text : & str | match alignment { Alignment :: None | Alignment :: Left => format ! (" {:<width$} " , text , width = width) , Alignment :: Center => format ! (" {:^width$} " , text , width = width) , Alignment :: Right => format ! (" {:>width$} " , text , width = width) , } ; if text . len () < width { vec ! [fill_width (text)] } else { let mut result = Vec :: new () ; let mut line = String :: new () ; for word in text . split_whitespace () { if line . len () + word . len () >= width { result . push (fill_width (& line)) ; line . clear () ; } if line . is_empty () { line . push_str (word) ; } else { line . push (' ') ; line . push_str (& word) ; } } if ! line . is_empty () { result . push (fill_width (& line)) ; } result } }
};
}
