// Generated macro for internal (module)
macro_rules! Depcrateinternal {
() => {
// Module: crate
// Provides: {"internal"}
// Dependencies: {}
# [doc = " Crate internals used by the `select!` macro."] # [doc (hidden)] # [cfg (feature = "std")] pub mod internal { pub use crate :: select :: { select , select_timeout , try_select , SelectHandle } ; }
};
}
