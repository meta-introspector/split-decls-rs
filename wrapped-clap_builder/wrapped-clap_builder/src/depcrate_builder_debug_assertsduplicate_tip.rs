// Generated macro for duplicate_tip (function)
macro_rules! Depcrate_builder_debug_assertsduplicate_tip {
() => {
// Module: crate::builder::debug_asserts
// Provides: {"duplicate_tip"}
// Dependencies: {}
fn duplicate_tip (cmd : & Command , first : & Arg , second : & Arg) -> & 'static str { if ! cmd . is_disable_help_flag_set () && (first . get_id () == Id :: HELP || second . get_id () == Id :: HELP) { " (call `cmd.disable_help_flag(true)` to remove the auto-generated `--help`)" } else if ! cmd . is_disable_version_flag_set () && (first . get_id () == Id :: VERSION || second . get_id () == Id :: VERSION) { " (call `cmd.disable_version_flag(true)` to remove the auto-generated `--version`)" } else { "" } }
};
}
