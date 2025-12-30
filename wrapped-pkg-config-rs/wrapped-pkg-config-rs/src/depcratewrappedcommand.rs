// Generated macro for WrappedCommand (struct)
macro_rules! DepcrateWrappedCommand {
() => {
// Module: crate
// Provides: {"WrappedCommand"}
// Dependencies: {}
# [doc = " Wrapper struct to polyfill methods introduced in 1.57 (`get_envs`, `get_args` etc)."] # [doc = " This is needed to reconstruct the pkg-config command for output in a copy-"] # [doc = " paste friendly format via `Display`."] struct WrappedCommand { inner : Command , program : OsString , env_vars : Vec < (OsString , OsString) > , args : Vec < OsString > , }
};
}
