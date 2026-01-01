/* FP:remove_storage_markers.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_storage_markers_USE_0001
/* FP:remove_storage_markers.rs-0002 */ use crate :: rustc_complete :: mir :: * ;
/* FP:remove_storage_markers.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_storage_markers_USE_0002
/* FP:remove_storage_markers.rs-0004 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:remove_storage_markers.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_storage_markers_USE_0003
/* FP:remove_storage_markers.rs-0006 */ use tracing :: trace ;
/* FP:remove_storage_markers.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_storage_markers_STRUCT_0004
/* FP:remove_storage_markers.rs-0008 */ pub (super) struct RemoveStorageMarkers ;
/* FP:remove_storage_markers.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_remove_storage_markers_IMPL_0005
/* FP:remove_storage_markers.rs-0010 */ impl < 'tcx > crate :: MirPass < 'tcx > for RemoveStorageMarkers { fn is_enabled (& self , sess : & crate :: rustc_session :: Session) -> bool { sess . mir_opt_level () > 0 && ! sess . emit_lifetime_markers () } fn run_pass (& self , _tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { trace ! ("Running RemoveStorageMarkers on {:?}" , body . source) ; for data in body . basic_blocks . as_mut_preserves_cfg () { data . statements . retain (| statement | match statement . kind { StatementKind :: StorageLive (..) | StatementKind :: StorageDead (..) | StatementKind :: Nop => false , _ => true , }) } } fn is_required (& self) -> bool { true } }