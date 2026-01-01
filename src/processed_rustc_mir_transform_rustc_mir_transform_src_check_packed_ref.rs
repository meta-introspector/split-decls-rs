/* FP:check_packed_ref.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_USE_0001
/* FP:check_packed_ref.rs-0002 */ use crate :: rustc_complete :: mir :: visit :: { PlaceContext , Visitor } ;
/* FP:check_packed_ref.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_USE_0002
/* FP:check_packed_ref.rs-0004 */ use crate :: rustc_complete :: mir :: * ;
/* FP:check_packed_ref.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_USE_0003
/* FP:check_packed_ref.rs-0006 */ use crate :: rustc_complete :: span_bug ;
/* FP:check_packed_ref.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_USE_0004
/* FP:check_packed_ref.rs-0008 */ use crate :: rustc_complete :: ty :: { self , TyCtxt } ;
/* FP:check_packed_ref.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_USE_0005
/* FP:check_packed_ref.rs-0010 */ use crate :: { errors , util } ;
/* FP:check_packed_ref.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_STRUCT_0006
/* FP:check_packed_ref.rs-0012 */ pub (super) struct CheckPackedRef ;
/* FP:check_packed_ref.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_IMPL_0007
/* FP:check_packed_ref.rs-0014 */ impl < 'tcx > crate :: MirLint < 'tcx > for CheckPackedRef { fn run_lint (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx >) { let typing_env = body . typing_env (tcx) ; let source_info = SourceInfo :: outermost (body . span) ; let mut checker = PackedRefChecker { body , tcx , typing_env , source_info } ; checker . visit_body (body) ; } }
/* FP:check_packed_ref.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_STRUCT_0008
/* FP:check_packed_ref.rs-0016 */ struct PackedRefChecker < 'a , 'tcx > { body : & 'a Body < 'tcx > , tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , source_info : SourceInfo , }
/* FP:check_packed_ref.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_check_packed_ref_IMPL_0009
/* FP:check_packed_ref.rs-0018 */ impl < 'tcx > Visitor < 'tcx > for PackedRefChecker < '_ , 'tcx > { fn visit_terminator (& mut self , terminator : & Terminator < 'tcx > , location : Location) { self . source_info = terminator . source_info ; self . super_terminator (terminator , location) ; } fn visit_statement (& mut self , statement : & Statement < 'tcx > , location : Location) { self . source_info = statement . source_info ; self . super_statement (statement , location) ; } fn visit_place (& mut self , place : & Place < 'tcx > , context : PlaceContext , _location : Location) { if context . is_borrow () && util :: is_disaligned (self . tcx , self . body , self . typing_env , * place) { let def_id = self . body . source . instance . def_id () ; if let Some (impl_def_id) = self . tcx . trait_impl_of_assoc (def_id) && self . tcx . is_builtin_derived (impl_def_id) { span_bug ! (self . source_info . span , "builtin derive created an unaligned reference") ; } else { self . tcx . dcx () . emit_err (errors :: UnalignedPackedRef { span : self . source_info . span }) ; } } } }