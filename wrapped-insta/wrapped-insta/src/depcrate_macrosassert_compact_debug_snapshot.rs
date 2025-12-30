// Generated macro for assert_compact_debug_snapshot (macro)
macro_rules! Depcrate_macrosassert_compact_debug_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_compact_debug_snapshot"}
// Dependencies: {}
# [doc = " Asserts a [`Debug`] snapshot in compact format."] # [doc = ""] # [doc = " The value needs to implement the [`Debug`] trait.  This is useful for"] # [doc = " simple values that do not implement the [`serde::Serialize`] trait, but does not"] # [doc = " permit redactions."] # [doc = ""] # [doc = " Debug is called with `\"{:?}\"`, which means this does not use pretty-print."] # [macro_export] macro_rules ! assert_compact_debug_snapshot { ($ ($ arg : tt) *) => { $ crate :: _assert_snapshot_base ! (transform =| v | $ crate :: _macro_support :: format ! ("{:?}" , v) , $ ($ arg) *) } ; }
};
}
