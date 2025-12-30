// Generated macro for impl_297 (impl)
macro_rules! Depcrate___macros_module_infoimpl_297 {
() => {
// Module: crate::__macros::module_info
// Provides: {"impl_297"}
// Dependencies: {}
impl ModuleInfo { # [doc = " This is hardcoded in clang as 7."] const VERSION : usize = 7 ; pub const fn new (name : * const u8) -> Self { Self { version : Self :: VERSION , size : core :: mem :: size_of :: < Self > () , name , symtab : core :: ptr :: null () , } } }
};
}
