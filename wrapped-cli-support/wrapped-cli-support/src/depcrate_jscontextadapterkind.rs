// Generated macro for ContextAdapterKind (enum)
macro_rules! Depcrate_jsContextAdapterKind {
() => {
// Module: crate::js
// Provides: {"ContextAdapterKind"}
// Dependencies: {}
# [doc = " A categorization of adapters for the purpose of code generation."] # [doc = ""] # [doc = " This is different from [`AdapterKind`] and is only used internally in the"] # [doc = " code generation process."] enum ContextAdapterKind < 'a > { # [doc = " An exported function, method, constrctor, or getter/setter."] Export (& 'a AuxExport) , # [doc = " An imported function or intrinsic."] Import (walrus :: ImportId) , Adapter , }
};
}
