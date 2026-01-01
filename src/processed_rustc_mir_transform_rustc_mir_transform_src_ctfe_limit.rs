/* FP:ctfe_limit.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_USE_0001
/* FP:ctfe_limit.rs-0002 */ use crate :: rustc_data_structures :: graph :: dominators :: Dominators ;
/* FP:ctfe_limit.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_USE_0002
/* FP:ctfe_limit.rs-0004 */ use crate :: rustc_complete :: mir :: { BasicBlock , BasicBlockData , Body , Statement , StatementKind , TerminatorKind , } ;
/* FP:ctfe_limit.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_USE_0003
/* FP:ctfe_limit.rs-0006 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:ctfe_limit.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_USE_0004
/* FP:ctfe_limit.rs-0008 */ use tracing :: instrument ;
/* FP:ctfe_limit.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_STRUCT_0005
/* FP:ctfe_limit.rs-0010 */ pub (super) struct CtfeLimit ;
/* FP:ctfe_limit.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_IMPL_0006
/* FP:ctfe_limit.rs-0012 */ impl < 'tcx > crate :: MirPass < 'tcx > for CtfeLimit { # [instrument (skip (self , _tcx , body))] fn run_pass (& self , _tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let doms = body . basic_blocks . dominators () ; let indices : Vec < BasicBlock > = body . basic_blocks . iter_enumerated () . filter_map (| (node , node_data) | { if matches ! (node_data . terminator () . kind , TerminatorKind :: Call { .. } | TerminatorKind :: TailCall { .. }) || has_back_edge (doms , node , node_data) { Some (node) } else { None } }) . collect () ; for index in indices { insert_counter (body . basic_blocks_mut () . get_mut (index) . expect ("basic_blocks index {index} should exist") ,) ; } } fn is_required (& self) -> bool { true } }
/* FP:ctfe_limit.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_FN_0007
/* FP:ctfe_limit.rs-0014 */ fn has_back_edge (doms : & Dominators < BasicBlock > , node : BasicBlock , node_data : & BasicBlockData < '_ > ,) -> bool { if ! doms . is_reachable (node) { return false ; } node_data . terminator () . successors () . any (| succ | doms . dominates (succ , node)) }
/* FP:ctfe_limit.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_ctfe_limit_FN_0008
/* FP:ctfe_limit.rs-0016 */ fn insert_counter (basic_block_data : & mut BasicBlockData < '_ >) { basic_block_data . statements . push (Statement :: new (basic_block_data . terminator () . source_info , StatementKind :: ConstEvalCounter ,)) ; }