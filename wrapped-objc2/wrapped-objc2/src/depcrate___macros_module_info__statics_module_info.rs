// Generated macro for __statics_module_info (macro)
macro_rules! Depcrate___macros_module_info__statics_module_info {
() => {
// Module: crate::__macros::module_info
// Provides: {"__statics_module_info"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (target_vendor = "apple")] macro_rules ! __statics_module_info { ($ hash : expr) => { # [link_section = "__TEXT,__cstring,cstring_literals"] # [export_name = $ crate :: __macros :: concat ! ("\x01L_OBJC_CLASS_NAME_" , $ hash , "_MODULE_INFO")] static MODULE_INFO_NAME : [$ crate :: __macros :: u8 ; 1] = [0] ; # [doc = " Emit module info."] # [doc = ""] # [doc = " This is similar to image info, and must be present in the final"] # [doc = " binary on macOS 32-bit."] # [link_section = "__OBJC,__module_info,regular,no_dead_strip"] # [export_name = $ crate :: __macros :: concat ! ("\x01L_OBJC_MODULES_" , $ hash)] # [used] static _MODULE_INFO : $ crate :: __macros :: ModuleInfo = $ crate :: __macros :: ModuleInfo :: new (MODULE_INFO_NAME . as_ptr ()) ; } ; }
};
}
