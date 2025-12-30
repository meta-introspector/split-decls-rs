// Generated macro for ModuleInfo (struct)
macro_rules! Depcrate___macros_module_infoModuleInfo {
() => {
// Module: crate::__macros::module_info
// Provides: {"ModuleInfo"}
// Dependencies: {}
# [doc = " Helper struct for emitting the module info that macOS 32-bit requires."] # [doc = ""] # [doc = " <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCMac.cpp#L5211-L5234>"] # [repr (C)] # [derive (Debug)] pub struct ModuleInfo { version : usize , size : usize , name : * const u8 , symtab : * const () , }
};
}
