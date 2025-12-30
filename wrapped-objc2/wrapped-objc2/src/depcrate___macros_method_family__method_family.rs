// Generated macro for __method_family (macro)
macro_rules! Depcrate___macros_method_family__method_family {
() => {
// Module: crate::__macros::method_family
// Provides: {"__method_family"}
// Dependencies: {}
# [doc = " Get the method family from an explicit family name, if specified,"] # [doc = " otherwise infer it from the given selector."] # [doc = ""] # [doc = " No validation of the selector is done here, that must be done elsewhere."] # [doc (hidden)] # [macro_export] macro_rules ! __method_family { (($ ($ method_family : tt) +) ($ ($ sel : tt) *)) => { $ crate :: __macros :: method_family_import ::$ ($ method_family) + } ; (() (alloc)) => { $ crate :: __macros :: AllocFamily } ; (() (new)) => { $ crate :: __macros :: NewFamily } ; (() (init)) => { $ crate :: __macros :: InitFamily } ; (() (dealloc)) => { $ crate :: __macros :: DeallocSelector } ; (() (retain)) => { $ crate :: __macros :: RetainSelector } ; (() (release)) => { $ crate :: __macros :: ReleaseSelector } ; (() (autorelease)) => { $ crate :: __macros :: AutoreleaseSelector } ; (() ($ sel_first : tt $ ($ sel_rest : tt) *)) => { $ crate :: __macros :: MethodFamily < { $ crate :: __macros :: method_family ($ crate :: __macros :: stringify ! ($ sel_first)) } > } ; (() ()) => { $ crate :: __macros :: MethodFamily < { $ crate :: __macros :: compile_error ! ("missing selector") ; $ crate :: __macros :: method_family ("") } > } ; }
};
}
