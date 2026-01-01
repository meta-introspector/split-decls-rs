/* FP:remove_place_mention.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_place_mention_USE_0001
/* FP:remove_place_mention.rs-0002 */ use crate :: rustc_complete :: mir :: * ;
/* FP:remove_place_mention.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_place_mention_USE_0002
/* FP:remove_place_mention.rs-0004 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:remove_place_mention.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_place_mention_USE_0003
/* FP:remove_place_mention.rs-0006 */ use tracing :: trace ;
/* FP:remove_place_mention.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_place_mention_STRUCT_0004
/* FP:remove_place_mention.rs-0008 */ pub (super) struct RemovePlaceMention ;
/* FP:remove_place_mention.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_place_mention_IMPL_0005
/* FP:remove_place_mention.rs-0010 */ impl < 'tcx > crate :: MirPass < 'tcx > for RemovePlaceMention { fn is_enabled (& self , sess : & crate :: rustc_session :: Session) -> bool { ! sess . opts . unstable_opts . mir_preserve_ub } fn run_pass (& self , _ : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { trace ! ("Running RemovePlaceMention on {:?}" , body . source) ; for data in body . basic_blocks . as_mut_preserves_cfg () { data . statements . retain (| statement | match statement . kind { StatementKind :: PlaceMention (..) | StatementKind :: Nop => false , _ => true , }) } } fn is_required (& self) -> bool { true } }