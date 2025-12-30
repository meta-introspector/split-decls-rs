// Generated macro for set_link_section (function)
macro_rules! Depcrate_baseset_link_section {
() => {
// Module: crate::base
// Provides: {"set_link_section"}
// Dependencies: {}
pub (crate) fn set_link_section (llval : & Value , attrs : & CodegenFnAttrs) { let Some (sect) = attrs . link_section else { return } ; let buf = SmallCStr :: new (sect . as_str ()) ; llvm :: set_section (llval , & buf) ; }
};
}
