// Generated macro for enable_vt_processing (function)
macro_rules! Depcrate_ansi_supportenable_vt_processing {
() => {
// Module: crate::ansi_support
// Provides: {"enable_vt_processing"}
// Dependencies: {}
# [doc = " Enable virtual terminal processing."] # [doc = ""] # [doc = " This method attempts to enable virtual terminal processing for this"] # [doc = " console. If there was a problem enabling it, then an error returned."] # [doc = " On success, the caller may assume that enabling it was successful."] # [doc = ""] # [doc = " When virtual terminal processing is enabled, characters emitted to the"] # [doc = " console are parsed for VT100 and similar control character sequences"] # [doc = " that control color and other similar operations."] fn enable_vt_processing () -> std :: io :: Result < () > { let mask = ENABLE_VIRTUAL_TERMINAL_PROCESSING ; let console_mode = ConsoleMode :: from (Handle :: current_out_handle () ?) ; let old_mode = console_mode . mode () ? ; if old_mode & mask == 0 { console_mode . set_mode (old_mode | mask) ? ; } Ok (()) }
};
}
