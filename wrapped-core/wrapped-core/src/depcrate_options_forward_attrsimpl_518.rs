// Generated macro for impl_518 (impl)
macro_rules! Depcrate_options_forward_attrsimpl_518 {
() => {
// Module: crate::options::forward_attrs
// Provides: {"impl_518"}
// Dependencies: {}
impl FromMeta for ForwardAttrsFilter { fn from_word () -> Result < Self > { Ok (ForwardAttrsFilter :: All) } fn from_list (nested : & [NestedMeta]) -> Result < Self > { Ok (ForwardAttrsFilter :: Only (PathList :: from_list (nested) ?)) } }
};
}
