// Generated macro for Cli (struct)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [derive (Parser)] # [command (after_help = "Environment variables:
  MIRI_SYSROOT: If already set, the \"sysroot setup\" step is skipped
  CARGO_EXTRA_FLAGS: Pass extra flags to all cargo invocations")] pub struct Cli { # [command (subcommand)] pub command : Command , }
};
}
