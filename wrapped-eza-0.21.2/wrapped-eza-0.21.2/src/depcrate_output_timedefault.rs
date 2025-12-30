// Generated macro for default (function)
macro_rules! Depcrate_output_timedefault {
() => {
// Module: crate::output::time
// Provides: {"default"}
// Dependencies: {}
fn default (time : & DateTime < FixedOffset >) -> String { let month = & * LOCALE . short_month_name (time . month0 () as usize) ; let month_width = short_month_padding (* MAX_MONTH_WIDTH , month) ; let format = if time . year () == * CURRENT_YEAR { format ! ("%_d {month:<month_width$} %H:%M") } else { format ! ("%_d {month:<month_width$}  %Y") } ; time . format (format . as_str ()) . to_string () }
};
}
