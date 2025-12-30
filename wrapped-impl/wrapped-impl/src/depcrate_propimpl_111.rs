// Generated macro for impl_111 (impl)
macro_rules! Depcrate_propimpl_111 {
() => {
// Module: crate::prop
// Provides: {"impl_111"}
// Dependencies: {}
impl Enum < '_ > { pub (crate) fn has_source (& self) -> bool { self . variants . iter () . any (| variant | variant . source_field () . is_some () || variant . attrs . transparent . is_some ()) } pub (crate) fn has_backtrace (& self) -> bool { self . variants . iter () . any (| variant | variant . backtrace_field () . is_some ()) } pub (crate) fn has_display (& self) -> bool { self . attrs . display . is_some () || self . attrs . transparent . is_some () || self . attrs . fmt . is_some () || self . variants . iter () . any (| variant | variant . attrs . display . is_some () || variant . attrs . fmt . is_some ()) || self . variants . iter () . all (| variant | variant . attrs . transparent . is_some ()) } }
};
}
