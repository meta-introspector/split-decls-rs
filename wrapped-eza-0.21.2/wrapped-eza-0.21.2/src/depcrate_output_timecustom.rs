// Generated macro for custom (function)
macro_rules! Depcrate_output_timecustom {
() => {
// Module: crate::output::time
// Provides: {"custom"}
// Dependencies: {}
fn custom (time : & DateTime < FixedOffset > , non_recent_fmt : & str , recent_fmt : Option < & str >) -> String { if let Some (recent_fmt) = recent_fmt { if time . year () == * CURRENT_YEAR { time . format (recent_fmt) . to_string () } else { time . format (non_recent_fmt) . to_string () } } else { time . format (non_recent_fmt) . to_string () } }
};
}
