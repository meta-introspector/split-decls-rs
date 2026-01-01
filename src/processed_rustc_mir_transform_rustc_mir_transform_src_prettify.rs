/* FP:prettify.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_USE_0001
/* FP:prettify.rs-0002 */ use crate :: rustc_index :: bit_set :: DenseBitSet ;
/* FP:prettify.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_USE_0002
/* FP:prettify.rs-0004 */ use crate :: rustc_index :: { IndexSlice , IndexVec } ;
/* FP:prettify.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_USE_0003
/* FP:prettify.rs-0006 */ use crate :: rustc_complete :: mir :: visit :: { MutVisitor , PlaceContext , Visitor } ;
/* FP:prettify.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_USE_0004
/* FP:prettify.rs-0008 */ use crate :: rustc_complete :: mir :: * ;
/* FP:prettify.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_USE_0005
/* FP:prettify.rs-0010 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:prettify.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_USE_0006
/* FP:prettify.rs-0012 */ use crate :: rustc_complete :: Session ;
/* FP:prettify.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_STRUCT_0007
/* FP:prettify.rs-0014 */ # [doc = " Rearranges the basic blocks into a *reverse post-order*."] # [doc = ""] # [doc = " Thus after this pass, all the successors of a block are later than it in the"] # [doc = " `IndexVec`, unless that successor is a back-edge (such as from a loop)."] pub (super) struct ReorderBasicBlocks ;
/* FP:prettify.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_IMPL_0008
/* FP:prettify.rs-0016 */ impl < 'tcx > crate :: MirPass < 'tcx > for ReorderBasicBlocks { fn is_enabled (& self , _session : & Session) -> bool { false } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let rpo : IndexVec < BasicBlock , BasicBlock > = body . basic_blocks . reverse_postorder () . iter () . copied () . collect () ; if rpo . iter () . is_sorted () { return ; } let mut updater = BasicBlockUpdater { map : rpo . invert_bijective_mapping () , tcx } ; debug_assert_eq ! (updater . map [START_BLOCK] , START_BLOCK) ; updater . visit_body (body) ; permute (body . basic_blocks . as_mut () , & updater . map) ; } fn is_required (& self) -> bool { false } }
/* FP:prettify.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_STRUCT_0009
/* FP:prettify.rs-0018 */ # [doc = " Rearranges the locals into *use* order."] # [doc = ""] # [doc = " Thus after this pass, a local with a smaller [`Location`] where it was first"] # [doc = " assigned or referenced will have a smaller number."] # [doc = ""] # [doc = " (Does not reorder arguments nor the [`RETURN_PLACE`].)"] pub (super) struct ReorderLocals ;
/* FP:prettify.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_IMPL_0010
/* FP:prettify.rs-0020 */ impl < 'tcx > crate :: MirPass < 'tcx > for ReorderLocals { fn is_enabled (& self , _session : & Session) -> bool { false } fn run_pass (& self , tcx : TyCtxt < 'tcx > , body : & mut Body < 'tcx >) { let mut finder = LocalFinder { map : IndexVec :: new () , seen : DenseBitSet :: new_empty (body . local_decls . len ()) , } ; for local in (0 ..= body . arg_count) . map (Local :: from_usize) { finder . track (local) ; } for (bb , bbd) in body . basic_blocks . iter_enumerated () { finder . visit_basic_block_data (bb , bbd) ; } for local in body . local_decls . indices () { finder . track (local) ; } if finder . map . iter () . is_sorted () { return ; } let mut updater = LocalUpdater { map : finder . map . invert_bijective_mapping () , tcx } ; for local in (0 ..= body . arg_count) . map (Local :: from_usize) { debug_assert_eq ! (updater . map [local] , local) ; } updater . visit_body_preserves_cfg (body) ; permute (& mut body . local_decls , & updater . map) ; } fn is_required (& self) -> bool { false } }
/* FP:prettify.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_FN_0011
/* FP:prettify.rs-0022 */ fn permute < I : crate :: rustc_index :: Idx + Ord , T > (data : & mut IndexVec < I , T > , map : & IndexSlice < I , I >) { let mut enumerated : Vec < _ > = std :: mem :: take (data) . into_iter_enumerated () . collect () ; enumerated . sort_by_key (| p | map [p . 0]) ; * data = enumerated . into_iter () . map (| p | p . 1) . collect () ; }
/* FP:prettify.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_STRUCT_0012
/* FP:prettify.rs-0024 */ struct BasicBlockUpdater < 'tcx > { map : IndexVec < BasicBlock , BasicBlock > , tcx : TyCtxt < 'tcx > , }
/* FP:prettify.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_IMPL_0013
/* FP:prettify.rs-0026 */ impl < 'tcx > MutVisitor < 'tcx > for BasicBlockUpdater < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_terminator (& mut self , terminator : & mut Terminator < 'tcx > , _location : Location) { terminator . successors_mut (| succ | * succ = self . map [* succ]) ; } }
/* FP:prettify.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_STRUCT_0014
/* FP:prettify.rs-0028 */ struct LocalFinder { map : IndexVec < Local , Local > , seen : DenseBitSet < Local > , }
/* FP:prettify.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_IMPL_0015
/* FP:prettify.rs-0030 */ impl LocalFinder { fn track (& mut self , l : Local) { if self . seen . insert (l) { self . map . push (l) ; } } }
/* FP:prettify.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_IMPL_0016
/* FP:prettify.rs-0032 */ impl < 'tcx > Visitor < 'tcx > for LocalFinder { fn visit_local (& mut self , l : Local , context : PlaceContext , _location : Location) { if context . is_use () { self . track (l) ; } } }
/* FP:prettify.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_STRUCT_0017
/* FP:prettify.rs-0034 */ struct LocalUpdater < 'tcx > { map : IndexVec < Local , Local > , tcx : TyCtxt < 'tcx > , }
/* FP:prettify.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_prettify_IMPL_0018
/* FP:prettify.rs-0036 */ impl < 'tcx > MutVisitor < 'tcx > for LocalUpdater < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_local (& mut self , l : & mut Local , _ : PlaceContext , _ : Location) { * l = self . map [* l] ; } }