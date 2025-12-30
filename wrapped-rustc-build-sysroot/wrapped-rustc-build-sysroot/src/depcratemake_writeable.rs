// Generated macro for make_writeable (function)
macro_rules! Depcratemake_writeable {
() => {
// Module: crate
// Provides: {"make_writeable"}
// Dependencies: {}
# [doc = " Make a file writeable."] # [cfg (not (unix))] fn make_writeable (p : & Path) -> Result < () > { let mut perms = fs :: metadata (p) ? . permissions () ; perms . set_readonly (false) ; fs :: set_permissions (p , perms) . context ("cannot set permissions") ? ; Ok (()) }
};
}
