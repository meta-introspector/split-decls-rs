// Generated macro for impl_55 (impl)
macro_rules! Depcrate_argsimpl_55 {
() => {
// Module: crate::args
// Provides: {"impl_55"}
// Dependencies: {}
impl RenameRuleExt for Option < RenameRule > { fn rename (& self , name : impl AsRef < str > , target : RenameTarget) -> String { self . unwrap_or (target . rule ()) . rename (name) } }
};
}
