// Generated macro for DebugInfoCodegenMethods (trait)
macro_rules! Depcrate_traits_debuginfoDebugInfoCodegenMethods {
() => {
// Module: crate::traits::debuginfo
// Provides: {"DebugInfoCodegenMethods"}
// Dependencies: {}
pub trait DebugInfoCodegenMethods < 'tcx > : BackendTypes { fn create_vtable_debuginfo (& self , ty : Ty < 'tcx > , trait_ref : Option < ExistentialTraitRef < 'tcx > > , vtable : Self :: Value ,) ; # [doc = " Creates the function-specific debug context."] # [doc = ""] # [doc = " Returns the FunctionDebugContext for the function which holds state needed"] # [doc = " for debug info creation, if it is enabled."] fn create_function_debug_context (& self , instance : Instance < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , llfn : Self :: Function , mir : & mir :: Body < 'tcx > ,) -> Option < FunctionDebugContext < 'tcx , Self :: DIScope , Self :: DILocation > > ; fn dbg_scope_fn (& self , instance : Instance < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , maybe_definition_llfn : Option < Self :: Function > ,) -> Self :: DIScope ; fn dbg_loc (& self , scope : Self :: DIScope , inlined_at : Option < Self :: DILocation > , span : Span ,) -> Self :: DILocation ; fn extend_scope_to_file (& self , scope_metadata : Self :: DIScope , file : & SourceFile ,) -> Self :: DIScope ; fn debuginfo_finalize (& self) ; fn create_dbg_var (& self , variable_name : Symbol , variable_type : Ty < 'tcx > , scope_metadata : Self :: DIScope , variable_kind : VariableKind , span : Span ,) -> Self :: DIVariable ; }
};
}
