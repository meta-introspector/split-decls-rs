// Generated macro for impl_260 (impl)
macro_rules! Depcrate_back_linkerimpl_260 {
() => {
// Module: crate::back::linker
// Provides: {"impl_260"}
// Dependencies: {}
impl dyn Linker + '_ { pub (crate) fn take_cmd (& mut self) -> Command { mem :: replace (self . cmd () , Command :: new ("")) } }
};
}
