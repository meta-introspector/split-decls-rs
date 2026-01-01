/* FP:util.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_thir_util_USE_0001
/* FP:util.rs-0002 */ use std :: assert_matches :: assert_matches ;
/* FP:util.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_thir_util_USE_0002
/* FP:util.rs-0004 */ use rustc_hir as hir ;
/* FP:util.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_thir_util_USE_0003
/* FP:util.rs-0006 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:util.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_thir_util_USE_0004
/* FP:util.rs-0008 */ use crate :: rustc_complete :: bug ;
/* FP:util.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_thir_util_USE_0005
/* FP:util.rs-0010 */ use crate :: rustc_complete :: ty :: { self , CanonicalUserType , TyCtxt } ;
/* FP:util.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_thir_util_USE_0006
/* FP:util.rs-0012 */ use tracing :: debug ;
/* FP:util.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_build_src_thir_util_FN_0007
/* FP:util.rs-0014 */ # [doc = " Looks up the type associated with this hir-id and applies the"] # [doc = " user-given generic parameters; the hir-id must map to a suitable"] # [doc = " type."] pub (crate) fn user_args_applied_to_ty_of_hir_id < 'tcx > (tcx : TyCtxt < 'tcx > , typeck_results : & ty :: TypeckResults < 'tcx > , hir_id : hir :: HirId ,) -> Option < CanonicalUserType < 'tcx > > { let user_provided_types = typeck_results . user_provided_types () ; let mut user_ty = * user_provided_types . get (hir_id) ? ; debug ! ("user_subts_applied_to_ty_of_hir_id: user_ty={:?}" , user_ty) ; let ty = typeck_results . node_type (hir_id) ; match ty . kind () { ty :: Adt (adt_def , ..) => { if let ty :: UserTypeKind :: TypeOf (did , _) = & mut user_ty . value . kind { assert_matches ! (tcx . def_kind (* did) , DefKind :: Ctor (..) | DefKind :: Struct | DefKind :: Enum | DefKind :: Union | DefKind :: Variant) ; * did = adt_def . did () ; } Some (user_ty) } ty :: FnDef (..) => Some (user_ty) , _ => bug ! ("ty: {:?} should not have user provided type {:?} recorded " , ty , user_ty) , } }