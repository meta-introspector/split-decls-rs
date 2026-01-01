/* FP:namespace.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_USE_0001
/* FP:namespace.rs-0002 */ use crate :: rustc_codegen_ssa :: debuginfo :: type_names ;
/* FP:namespace.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_USE_0002
/* FP:namespace.rs-0004 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:namespace.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_USE_0003
/* FP:namespace.rs-0006 */ use crate :: rustc_complete :: ty :: { self , Instance } ;
/* FP:namespace.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_USE_0004
/* FP:namespace.rs-0008 */ use super :: utils :: { DIB , debug_context } ;
/* FP:namespace.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_USE_0005
/* FP:namespace.rs-0010 */ use crate :: common :: CodegenCx ;
/* FP:namespace.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_USE_0006
/* FP:namespace.rs-0012 */ use crate :: llvm ;
/* FP:namespace.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_USE_0007
/* FP:namespace.rs-0014 */ use crate :: llvm :: debuginfo :: DIScope ;
/* FP:namespace.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_FN_0008
/* FP:namespace.rs-0016 */ pub (crate) fn mangled_name_of_instance < 'a , 'tcx > (cx : & CodegenCx < 'a , 'tcx > , instance : Instance < 'tcx > ,) -> ty :: SymbolName < 'tcx > { cx . tcx . symbol_name (instance) }
/* FP:namespace.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_namespace_FN_0009
/* FP:namespace.rs-0018 */ pub (crate) fn item_namespace < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId) -> & 'll DIScope { if let Some (& scope) = debug_context (cx) . namespace_map . borrow () . get (& def_id) { return scope ; } let def_key = cx . tcx . def_key (def_id) ; let parent_scope = def_key . parent . map (| parent | item_namespace (cx , DefId { krate : def_id . krate , index : parent })) ; let namespace_name_string = { let mut output = String :: with_capacity (64) ; type_names :: push_item_name (cx . tcx , def_id , false , & mut output) ; output } ; let scope = unsafe { llvm :: LLVMDIBuilderCreateNameSpace (DIB (cx) , parent_scope , namespace_name_string . as_ptr () , namespace_name_string . len () , llvm :: FALSE ,) } ; debug_context (cx) . namespace_map . borrow_mut () . insert (def_id , scope) ; scope }