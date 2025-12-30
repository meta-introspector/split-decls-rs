// Generated macro for render_date (function)
macro_rules! Depcraterender_date {
() => {
// Module: crate
// Provides: {"render_date"}
// Dependencies: {}
# [doc = " Render the date with the `:` flashing on and off every second into `el`."] fn render_date (el : & web_sys :: Element) { let date = chrono :: Local :: now () ; let format_str = if date . second () % 2 == 0 { "%Y-%m-%d %H %M" } else { "%Y-%m-%d %H:%M" } ; let date_str = date . format (format_str) . to_string () ; el . set_text_content (Some (& date_str)) ; }
};
}
