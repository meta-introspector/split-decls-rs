/* FP:basic_blocks.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0001
/* FP:basic_blocks.rs-0002 */ use std :: sync :: OnceLock ;
/* FP:basic_blocks.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0002
/* FP:basic_blocks.rs-0004 */ use crate :: rustc_data_structures :: graph ;
/* FP:basic_blocks.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0003
/* FP:basic_blocks.rs-0006 */ use crate :: rustc_data_structures :: graph :: dominators :: { Dominators , dominators } ;
/* FP:basic_blocks.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0004
/* FP:basic_blocks.rs-0008 */ use crate :: rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;
/* FP:basic_blocks.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0005
/* FP:basic_blocks.rs-0010 */ use crate :: rustc_index :: { IndexSlice , IndexVec } ;
/* FP:basic_blocks.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0006
/* FP:basic_blocks.rs-0012 */ use rustc_macros :: { HashStable , TyDecodable , TyEncodable , TypeFoldable , TypeVisitable } ;
/* FP:basic_blocks.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0007
/* FP:basic_blocks.rs-0014 */ use crate :: rustc_serialize :: { Decodable , Decoder , Encodable , Encoder } ;
/* FP:basic_blocks.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0008
/* FP:basic_blocks.rs-0016 */ use smallvec :: SmallVec ;
/* FP:basic_blocks.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0009
/* FP:basic_blocks.rs-0018 */ use crate :: mir :: traversal :: Postorder ;
/* FP:basic_blocks.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_USE_0010
/* FP:basic_blocks.rs-0020 */ use crate :: mir :: { BasicBlock , BasicBlockData , START_BLOCK } ;
/* FP:basic_blocks.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_STRUCT_0011
/* FP:basic_blocks.rs-0022 */ # [derive (Clone , TyEncodable , TyDecodable , Debug , HashStable , TypeFoldable , TypeVisitable)] pub struct BasicBlocks < 'tcx > { basic_blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > > , cache : Cache , }
/* FP:basic_blocks.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_TYPE_0012
/* FP:basic_blocks.rs-0024 */ type Predecessors = IndexVec < BasicBlock , SmallVec < [BasicBlock ; 4] > > ;
/* FP:basic_blocks.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_ENUM_0013
/* FP:basic_blocks.rs-0026 */ # [derive (Debug , Clone , Copy)] pub enum SwitchTargetValue { Normal (u128) , Otherwise , }
/* FP:basic_blocks.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_STRUCT_0014
/* FP:basic_blocks.rs-0028 */ # [derive (Clone , Default , Debug)] struct Cache { predecessors : OnceLock < Predecessors > , reverse_postorder : OnceLock < Vec < BasicBlock > > , dominators : OnceLock < Dominators < BasicBlock > > , }
/* FP:basic_blocks.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0015
/* FP:basic_blocks.rs-0030 */ impl < 'tcx > BasicBlocks < 'tcx > { # [inline] pub fn new (basic_blocks : IndexVec < BasicBlock , BasicBlockData < 'tcx > >) -> Self { BasicBlocks { basic_blocks , cache : Cache :: default () } } pub fn dominators (& self) -> & Dominators < BasicBlock > { self . cache . dominators . get_or_init (| | dominators (self)) } # [doc = " Returns predecessors for each basic block."] # [inline] pub fn predecessors (& self) -> & Predecessors { self . cache . predecessors . get_or_init (| | { let mut preds = IndexVec :: from_elem (SmallVec :: new () , & self . basic_blocks) ; for (bb , data) in self . basic_blocks . iter_enumerated () { if let Some (term) = & data . terminator { for succ in term . successors () { preds [succ] . push (bb) ; } } } preds }) } # [doc = " Returns basic blocks in a reverse postorder."] # [doc = ""] # [doc = " See [`traversal::reverse_postorder`]'s docs to learn what is preorder traversal."] # [doc = ""] # [doc = " [`traversal::reverse_postorder`]: crate::mir::traversal::reverse_postorder"] # [inline] pub fn reverse_postorder (& self) -> & [BasicBlock] { self . cache . reverse_postorder . get_or_init (| | { let mut rpo : Vec < _ > = Postorder :: new (& self . basic_blocks , START_BLOCK , None) . collect () ; rpo . reverse () ; rpo }) } # [doc = " Returns mutable reference to basic blocks. Invalidates CFG cache."] # [inline] pub fn as_mut (& mut self) -> & mut IndexVec < BasicBlock , BasicBlockData < 'tcx > > { self . invalidate_cfg_cache () ; & mut self . basic_blocks } # [doc = " Get mutable access to basic blocks without invalidating the CFG cache."] # [doc = ""] # [doc = " By calling this method instead of e.g. [`BasicBlocks::as_mut`] you promise not to change"] # [doc = " the CFG. This means that"] # [doc = ""] # [doc = "  1) The number of basic blocks remains unchanged"] # [doc = "  2) The set of successors of each terminator remains unchanged."] # [doc = "  3) For each `TerminatorKind::SwitchInt`, the `targets` remains the same and the terminator"] # [doc = "     kind is not changed."] # [doc = ""] # [doc = " If any of these conditions cannot be upheld, you should call [`BasicBlocks::invalidate_cfg_cache`]."] # [inline] pub fn as_mut_preserves_cfg (& mut self) -> & mut IndexVec < BasicBlock , BasicBlockData < 'tcx > > { & mut self . basic_blocks } # [doc = " Invalidates cached information about the CFG."] # [doc = ""] # [doc = " You will only ever need this if you have also called [`BasicBlocks::as_mut_preserves_cfg`]."] # [doc = " All other methods that allow you to mutate the basic blocks also call this method"] # [doc = " themselves, thereby avoiding any risk of accidentally cache invalidation."] pub fn invalidate_cfg_cache (& mut self) { self . cache = Cache :: default () ; } }
/* FP:basic_blocks.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0016
/* FP:basic_blocks.rs-0032 */ impl < 'tcx > std :: ops :: Deref for BasicBlocks < 'tcx > { type Target = IndexSlice < BasicBlock , BasicBlockData < 'tcx > > ; # [inline] fn deref (& self) -> & IndexSlice < BasicBlock , BasicBlockData < 'tcx > > { & self . basic_blocks } }
/* FP:basic_blocks.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0017
/* FP:basic_blocks.rs-0034 */ impl < 'tcx > graph :: DirectedGraph for BasicBlocks < 'tcx > { type Node = BasicBlock ; # [inline] fn num_nodes (& self) -> usize { self . basic_blocks . len () } }
/* FP:basic_blocks.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0018
/* FP:basic_blocks.rs-0036 */ impl < 'tcx > graph :: StartNode for BasicBlocks < 'tcx > { # [inline] fn start_node (& self) -> Self :: Node { START_BLOCK } }
/* FP:basic_blocks.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0019
/* FP:basic_blocks.rs-0038 */ impl < 'tcx > graph :: Successors for BasicBlocks < 'tcx > { # [inline] fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . basic_blocks [node] . terminator () . successors () } }
/* FP:basic_blocks.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0020
/* FP:basic_blocks.rs-0040 */ impl < 'tcx > graph :: Predecessors for BasicBlocks < 'tcx > { # [inline] fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . predecessors () [node] . iter () . copied () } }
/* FP:basic_blocks.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_MACRO_0021
/* FP:basic_blocks.rs-0042 */ TrivialTypeTraversalImpls ! { Cache }
/* FP:basic_blocks.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0022
/* FP:basic_blocks.rs-0044 */ impl < S : Encoder > Encodable < S > for Cache { # [inline] fn encode (& self , _s : & mut S) { } }
/* FP:basic_blocks.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0023
/* FP:basic_blocks.rs-0046 */ impl < D : Decoder > Decodable < D > for Cache { # [inline] fn decode (_ : & mut D) -> Self { Default :: default () } }
/* FP:basic_blocks.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_mir_basic_blocks_IMPL_0024
/* FP:basic_blocks.rs-0048 */ impl < CTX > HashStable < CTX > for Cache { # [inline] fn hash_stable (& self , _ : & mut CTX , _ : & mut StableHasher) { } }