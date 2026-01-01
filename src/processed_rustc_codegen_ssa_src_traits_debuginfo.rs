/* FP:debuginfo.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0001
/* FP:debuginfo.rs-0002 */ use std :: ops :: Range ;
/* FP:debuginfo.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0002
/* FP:debuginfo.rs-0004 */ use crate :: rustc_abi :: Size ;
/* FP:debuginfo.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0003
/* FP:debuginfo.rs-0006 */ use crate :: rustc_complete :: mir ;
/* FP:debuginfo.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0004
/* FP:debuginfo.rs-0008 */ use crate :: rustc_complete :: ty :: { ExistentialTraitRef , Instance , Ty } ;
/* FP:debuginfo.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0005
/* FP:debuginfo.rs-0010 */ use crate :: rustc_complete :: { SourceFile , Span , Symbol } ;
/* FP:debuginfo.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0006
/* FP:debuginfo.rs-0012 */ use crate :: rustc_target :: callconv :: FnAbi ;
/* FP:debuginfo.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0007
/* FP:debuginfo.rs-0014 */ use super :: BackendTypes ;
/* FP:debuginfo.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_USE_0008
/* FP:debuginfo.rs-0016 */ use crate :: mir :: debuginfo :: { FunctionDebugContext , VariableKind } ;
/* FP:debuginfo.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_TRAIT_0009
/* FP:debuginfo.rs-0018 */ pub trait DebugInfoCodegenMethods < 'tcx > : BackendTypes { fn create_vtable_debuginfo (& self , ty : Ty < 'tcx > , trait_ref : Option < ExistentialTraitRef < 'tcx > > , vtable : Self :: Value ,) ; # [doc = " Creates the function-specific debug context."] # [doc = ""] # [doc = " Returns the FunctionDebugContext for the function which holds state needed"] # [doc = " for debug info creation, if it is enabled."] fn create_function_debug_context (& self , instance : Instance < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , llfn : Self :: Function , mir : & mir :: Body < 'tcx > ,) -> Option < FunctionDebugContext < 'tcx , Self :: DIScope , Self :: DILocation > > ; fn dbg_scope_fn (& self , instance : Instance < 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , maybe_definition_llfn : Option < Self :: Function > ,) -> Self :: DIScope ; fn dbg_loc (& self , scope : Self :: DIScope , inlined_at : Option < Self :: DILocation > , span : Span ,) -> Self :: DILocation ; fn extend_scope_to_file (& self , scope_metadata : Self :: DIScope , file : & SourceFile ,) -> Self :: DIScope ; fn debuginfo_finalize (& self) ; fn create_dbg_var (& self , variable_name : Symbol , variable_type : Ty < 'tcx > , scope_metadata : Self :: DIScope , variable_kind : VariableKind , span : Span ,) -> Self :: DIVariable ; }
/* FP:debuginfo.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_ssa_src_traits_debuginfo_TRAIT_0010
/* FP:debuginfo.rs-0020 */ pub trait DebugInfoBuilderMethods : BackendTypes { fn dbg_var_addr (& mut self , dbg_var : Self :: DIVariable , dbg_loc : Self :: DILocation , variable_alloca : Self :: Value , direct_offset : Size , indirect_offsets : & [Size] , fragment : Option < Range < Size > > ,) ; fn set_dbg_loc (& mut self , dbg_loc : Self :: DILocation) ; fn clear_dbg_loc (& mut self) ; fn insert_reference_to_gdb_debug_scripts_section_global (& mut self) ; fn set_var_name (& mut self , value : Self :: Value , name : & str) ; }