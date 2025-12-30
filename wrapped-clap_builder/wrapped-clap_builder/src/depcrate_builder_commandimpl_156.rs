// Generated macro for impl_156 (impl)
macro_rules! Depcrate_builder_commandimpl_156 {
() => {
// Module: crate::builder::command
// Provides: {"impl_156"}
// Dependencies: {}
impl Index < & '_ Id > for Command { type Output = Arg ; fn index (& self , key : & Id) -> & Self :: Output { self . find (key) . expect (INTERNAL_ERROR_MSG) } }
};
}
