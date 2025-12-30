// Generated macro for __statics_class (macro)
macro_rules! Depcrate___macros_class__statics_class {
() => {
// Module: crate::__macros::class
// Provides: {"__statics_class"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (all (target_vendor = "apple" , all (target_os = "macos" , target_arch = "x86")))] macro_rules ! __statics_class { { ($ name : expr) ($ hash : expr) } => { const X : & [$ crate :: __macros :: u8] = $ name . as_bytes () ; # [doc = " Similar to NAME_DATA in `__statics_sel!`."] # [link_section = "__TEXT,__cstring,cstring_literals"] # [export_name = $ crate :: __macros :: concat ! ("\x01L_OBJC_CLASS_NAME_" , $ hash ,)] static NAME_DATA : [$ crate :: __macros :: u8 ; X . len ()] = $ crate :: __statics_string_to_known_length_bytes ! (X) ; # [doc = " SAFETY: Same as `REF` in `__statics_sel!`."] # [link_section = "__OBJC,__cls_refs,literal_pointers"] # [export_name = $ crate :: __macros :: concat ! ("\x01L_OBJC_CLASS_REFERENCES_" , $ hash ,)] static REF : $ crate :: __macros :: SyncUnsafeCell <&$ crate :: runtime :: AnyClass > = unsafe { let ptr : * const $ crate :: runtime :: AnyClass = NAME_DATA . as_ptr () . cast () ; $ crate :: __macros :: SyncUnsafeCell :: new (&* ptr) } ; $ crate :: __statics_image_info ! ($ hash) ; $ crate :: __statics_module_info ! ($ hash) ; } }
};
}
