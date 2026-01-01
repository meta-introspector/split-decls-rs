/* FP:cache.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_USE_0001
/* FP:cache.rs-0002 */ use std :: hash :: Hash ;
/* FP:cache.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_USE_0002
/* FP:cache.rs-0004 */ use crate :: rustc_data_structures :: fx :: FxHashMap ;
/* FP:cache.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_USE_0003
/* FP:cache.rs-0006 */ use crate :: rustc_data_structures :: sync :: Lock ;
/* FP:cache.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_USE_0004
/* FP:cache.rs-0008 */ use crate :: dep_graph :: { DepContext , DepNodeIndex } ;
/* FP:cache.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_STRUCT_0005
/* FP:cache.rs-0010 */ pub struct Cache < Key , Value > { hashmap : Lock < FxHashMap < Key , WithDepNode < Value > > > , }
/* FP:cache.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_IMPL_0006
/* FP:cache.rs-0012 */ impl < Key : Clone , Value : Clone > Clone for Cache < Key , Value > { fn clone (& self) -> Self { Self { hashmap : Lock :: new (self . hashmap . borrow () . clone ()) } } }
/* FP:cache.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_IMPL_0007
/* FP:cache.rs-0014 */ impl < Key , Value > Default for Cache < Key , Value > { fn default () -> Self { Self { hashmap : Default :: default () } } }
/* FP:cache.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_IMPL_0008
/* FP:cache.rs-0016 */ impl < Key , Value > Cache < Key , Value > { # [doc = " Actually frees the underlying memory in contrast to what stdlib containers do on `clear`"] pub fn clear (& self) { * self . hashmap . borrow_mut () = Default :: default () ; } }
/* FP:cache.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_IMPL_0009
/* FP:cache.rs-0018 */ impl < Key : Eq + Hash , Value : Clone > Cache < Key , Value > { pub fn get < Tcx : DepContext > (& self , key : & Key , tcx : Tcx) -> Option < Value > { Some (self . hashmap . borrow () . get (key) ? . get (tcx)) } pub fn insert (& self , key : Key , dep_node : DepNodeIndex , value : Value) { self . hashmap . borrow_mut () . insert (key , WithDepNode :: new (dep_node , value)) ; } }
/* FP:cache.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_STRUCT_0010
/* FP:cache.rs-0020 */ # [derive (Debug , Clone , Eq , PartialEq)] pub struct WithDepNode < T > { dep_node : DepNodeIndex , cached_value : T , }
/* FP:cache.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_cache_IMPL_0011
/* FP:cache.rs-0022 */ impl < T : Clone > WithDepNode < T > { pub fn new (dep_node : DepNodeIndex , cached_value : T) -> Self { WithDepNode { dep_node , cached_value } } pub fn get < Tcx : DepContext > (& self , tcx : Tcx) -> T { tcx . dep_graph () . read_index (self . dep_node) ; self . cached_value . clone () } }