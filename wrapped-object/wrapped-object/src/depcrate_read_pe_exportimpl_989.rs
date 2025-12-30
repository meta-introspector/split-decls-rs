// Generated macro for impl_989 (impl)
macro_rules! Depcrate_read_pe_exportimpl_989 {
() => {
// Module: crate::read::pe::export
// Provides: {"impl_989"}
// Dependencies: {}
impl < 'data > ExportTarget < 'data > { # [doc = " Returns true if the target is an address."] pub fn is_address (& self) -> bool { match self { ExportTarget :: Address (_) => true , _ => false , } } # [doc = " Returns true if the export is forwarded to another DLL."] pub fn is_forward (& self) -> bool { ! self . is_address () } }
};
}
