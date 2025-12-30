// Generated macro for messages (module)
macro_rules! Depcrate_errormessages {
() => {
// Module: crate::error
// Provides: {"messages"}
// Dependencies: {}
pub mod messages { pub const DESTRUCT_WITHOUT_FROM : & str = "To destruct a fixture you should provide a path to resolve it by '#[from(...)]' attribute." ; pub fn use_more_than_once (name : & str) -> String { format ! ("You cannot use '{name}' attribute more than once for the same argument") } }
};
}
