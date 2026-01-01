/* FP:node_id.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_node_id_USE_0001
/* FP:node_id.rs-0002 */ use std :: fmt ;
/* FP:node_id.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_node_id_USE_0002
/* FP:node_id.rs-0004 */ use crate :: rustc_complete :: LocalExpnId ;
/* FP:node_id.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_node_id_MACRO_0003
/* FP:node_id.rs-0006 */ crate :: rustc_index :: newtype_index ! { # [doc = " Identifies an AST node."] # [doc = ""] # [doc = " This identifies top-level definitions, expressions, and everything in between."] # [doc = " This is later turned into [`DefId`] and `HirId` for the HIR."] # [doc = ""] # [doc = " [`DefId`]: crate::rustc_span::def_id::DefId"] # [encodable] # [orderable] # [debug_format = "NodeId({})"] pub struct NodeId { # [doc = " The [`NodeId`] used to represent the root of the crate."] const CRATE_NODE_ID = 0 ; } }
/* FP:node_id.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_node_id_MACRO_0004
/* FP:node_id.rs-0008 */ crate :: rustc_data_structures :: define_id_collections ! (NodeMap , NodeSet , NodeMapEntry , NodeId) ;
/* FP:node_id.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_node_id_CONST_0005
/* FP:node_id.rs-0010 */ # [doc = " When parsing and at the beginning of doing expansions, we initially give all AST nodes"] # [doc = " this dummy AST [`NodeId`]. Then, during a later phase of expansion, we renumber them"] # [doc = " to have small, positive IDs."] pub const DUMMY_NODE_ID : NodeId = NodeId :: MAX ;
/* FP:node_id.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_node_id_IMPL_0006
/* FP:node_id.rs-0012 */ impl NodeId { pub fn placeholder_from_expn_id (expn_id : LocalExpnId) -> Self { NodeId :: from_u32 (expn_id . as_u32 ()) } pub fn placeholder_to_expn_id (self) -> LocalExpnId { LocalExpnId :: from_u32 (self . as_u32 ()) } }
/* FP:node_id.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_ast_src_node_id_IMPL_0007
/* FP:node_id.rs-0014 */ impl fmt :: Display for NodeId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . as_u32 () , f) } }