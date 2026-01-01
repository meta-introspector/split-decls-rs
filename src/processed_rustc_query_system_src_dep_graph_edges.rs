/* FP:edges.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_USE_0001
/* FP:edges.rs-0002 */ use std :: hash :: { Hash , Hasher } ;
/* FP:edges.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_USE_0002
/* FP:edges.rs-0004 */ use std :: ops :: Deref ;
/* FP:edges.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_USE_0003
/* FP:edges.rs-0006 */ use smallvec :: SmallVec ;
/* FP:edges.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_USE_0004
/* FP:edges.rs-0008 */ use crate :: dep_graph :: DepNodeIndex ;
/* FP:edges.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_STRUCT_0005
/* FP:edges.rs-0010 */ # [derive (Default , Debug)] pub (crate) struct EdgesVec { max : u32 , edges : SmallVec < [DepNodeIndex ; EdgesVec :: INLINE_CAPACITY] > , }
/* FP:edges.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_IMPL_0006
/* FP:edges.rs-0012 */ impl Hash for EdgesVec { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { Hash :: hash (& self . edges , hasher) } }
/* FP:edges.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_IMPL_0007
/* FP:edges.rs-0014 */ impl EdgesVec { pub (crate) const INLINE_CAPACITY : usize = 8 ; # [inline] pub (crate) fn new () -> Self { Self :: default () } # [inline] pub (crate) fn push (& mut self , edge : DepNodeIndex) { self . max = self . max . max (edge . as_u32 ()) ; self . edges . push (edge) ; } # [inline] pub (crate) fn max_index (& self) -> u32 { self . max } }
/* FP:edges.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_IMPL_0008
/* FP:edges.rs-0016 */ impl Deref for EdgesVec { type Target = [DepNodeIndex] ; # [inline] fn deref (& self) -> & Self :: Target { self . edges . as_slice () } }
/* FP:edges.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_IMPL_0009
/* FP:edges.rs-0018 */ impl FromIterator < DepNodeIndex > for EdgesVec { # [inline] fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = DepNodeIndex > , { let mut vec = EdgesVec :: new () ; for index in iter { vec . push (index) } vec } }
/* FP:edges.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_edges_IMPL_0010
/* FP:edges.rs-0020 */ impl Extend < DepNodeIndex > for EdgesVec { # [inline] fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = DepNodeIndex > , { for elem in iter { self . push (elem) ; } } }