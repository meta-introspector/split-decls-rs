/* FP:utils.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0001
/* FP:utils.rs-0002 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:utils.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0002
/* FP:utils.rs-0004 */ use crate :: rustc_complete :: ty :: layout :: { HasTypingEnv , LayoutOf } ;
/* FP:utils.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0003
/* FP:utils.rs-0006 */ use crate :: rustc_complete :: ty :: { self , Ty } ;
/* FP:utils.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0004
/* FP:utils.rs-0008 */ use tracing :: trace ;
/* FP:utils.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0005
/* FP:utils.rs-0010 */ use super :: CodegenUnitDebugContext ;
/* FP:utils.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0006
/* FP:utils.rs-0012 */ use super :: namespace :: item_namespace ;
/* FP:utils.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0007
/* FP:utils.rs-0014 */ use crate :: common :: CodegenCx ;
/* FP:utils.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0008
/* FP:utils.rs-0016 */ use crate :: llvm ;
/* FP:utils.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_USE_0009
/* FP:utils.rs-0018 */ use crate :: llvm :: debuginfo :: { DIArray , DIBuilder , DIDescriptor , DIScope } ;
/* FP:utils.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_FN_0010
/* FP:utils.rs-0020 */ pub (crate) fn is_node_local_to_unit (cx : & CodegenCx < '_ , '_ > , def_id : DefId) -> bool { ! cx . tcx . is_reachable_non_generic (def_id) }
/* FP:utils.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_FN_0011
/* FP:utils.rs-0022 */ # [allow (non_snake_case)] pub (crate) fn create_DIArray < 'll > (builder : & DIBuilder < 'll > , arr : & [Option < & 'll DIDescriptor >] ,) -> & 'll DIArray { unsafe { llvm :: LLVMRustDIBuilderGetOrCreateArray (builder , arr . as_ptr () , arr . len () as u32) } }
/* FP:utils.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_FN_0012
/* FP:utils.rs-0024 */ # [inline] pub (crate) fn debug_context < 'a , 'll , 'tcx > (cx : & 'a CodegenCx < 'll , 'tcx > ,) -> & 'a CodegenUnitDebugContext < 'll , 'tcx > { cx . dbg_cx . as_ref () . unwrap () }
/* FP:utils.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_FN_0013
/* FP:utils.rs-0026 */ # [inline] # [allow (non_snake_case)] pub (crate) fn DIB < 'a , 'll > (cx : & 'a CodegenCx < 'll , '_ >) -> & 'a DIBuilder < 'll > { cx . dbg_cx . as_ref () . unwrap () . builder . as_ref () }
/* FP:utils.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_FN_0014
/* FP:utils.rs-0028 */ pub (crate) fn get_namespace_for_item < 'll > (cx : & CodegenCx < 'll , '_ > , def_id : DefId) -> & 'll DIScope { item_namespace (cx , cx . tcx . parent (def_id)) }
/* FP:utils.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_ENUM_0015
/* FP:utils.rs-0030 */ # [derive (Debug , PartialEq , Eq)] pub (crate) enum WidePtrKind { Slice , Dyn , }
/* FP:utils.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_llvm_src_debuginfo_utils_FN_0016
/* FP:utils.rs-0032 */ # [doc = " Determines if `pointee_ty` is slice-like or trait-object-like, i.e."] # [doc = " if the second field of the wide pointer is a length or a vtable-pointer."] # [doc = " If `pointee_ty` does not require a wide pointer (because it is Sized) then"] # [doc = " the function returns `None`."] pub (crate) fn wide_pointer_kind < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , pointee_ty : Ty < 'tcx > ,) -> Option < WidePtrKind > { let pointee_tail_ty = cx . tcx . struct_tail_for_codegen (pointee_ty , cx . typing_env ()) ; let layout = cx . layout_of (pointee_tail_ty) ; trace ! ("wide_pointer_kind: {:?} has layout {:?} (is_unsized? {})" , pointee_tail_ty , layout , layout . is_unsized ()) ; if layout . is_sized () { return None ; } match * pointee_tail_ty . kind () { ty :: Str | ty :: Slice (_) => Some (WidePtrKind :: Slice) , ty :: Dynamic (..) => Some (WidePtrKind :: Dyn) , ty :: Foreign (_) => { assert_eq ! (cx . size_of (Ty :: new_imm_ptr (cx . tcx , pointee_tail_ty)) , cx . size_of (Ty :: new_imm_ptr (cx . tcx , cx . tcx . types . u8))) ; None } _ => { panic ! ("wide_pointer_kind() - Encountered unexpected `pointee_tail_ty`: {pointee_tail_ty:?}") } } }