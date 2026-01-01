/* FP:config.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0001
/* FP:config.rs-0002 */ use std :: fmt :: Debug ;
/* FP:config.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0002
/* FP:config.rs-0004 */ use std :: hash :: Hash ;
/* FP:config.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0003
/* FP:config.rs-0006 */ use crate :: rustc_data_structures :: fingerprint :: Fingerprint ;
/* FP:config.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0004
/* FP:config.rs-0008 */ use crate :: rustc_complete :: ErrorGuaranteed ;
/* FP:config.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0005
/* FP:config.rs-0010 */ use super :: QueryStackFrameExtra ;
/* FP:config.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0006
/* FP:config.rs-0012 */ use crate :: dep_graph :: { DepKind , DepNode , DepNodeParams , SerializedDepNodeIndex } ;
/* FP:config.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0007
/* FP:config.rs-0014 */ use crate :: error :: HandleCycleError ;
/* FP:config.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0008
/* FP:config.rs-0016 */ use crate :: ich :: StableHashingContext ;
/* FP:config.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0009
/* FP:config.rs-0018 */ use crate :: query :: caches :: QueryCache ;
/* FP:config.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_USE_0010
/* FP:config.rs-0020 */ use crate :: query :: { CycleError , DepNodeIndex , QueryContext , QueryState } ;
/* FP:config.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_TYPE_0011
/* FP:config.rs-0022 */ pub type HashResult < V > = Option < fn (& mut StableHashingContext < '_ > , & V) -> Fingerprint > ;
/* FP:config.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_config_TRAIT_0012
/* FP:config.rs-0024 */ pub trait QueryConfig < Qcx : QueryContext > : Copy { fn name (self) -> & 'static str ; type Key : DepNodeParams < Qcx :: DepContext > + Eq + Hash + Copy + Debug ; type Value : Copy ; type Cache : QueryCache < Key = Self :: Key , Value = Self :: Value > ; fn format_value (self) -> fn (& Self :: Value) -> String ; fn query_state < 'a > (self , tcx : Qcx) -> & 'a QueryState < Self :: Key , Qcx :: QueryInfo > where Qcx : 'a ; fn query_cache < 'a > (self , tcx : Qcx) -> & 'a Self :: Cache where Qcx : 'a ; fn cache_on_disk (self , tcx : Qcx :: DepContext , key : & Self :: Key) -> bool ; fn execute_query (self , tcx : Qcx :: DepContext , k : Self :: Key) -> Self :: Value ; fn compute (self , tcx : Qcx , key : Self :: Key) -> Self :: Value ; fn try_load_from_disk (self , tcx : Qcx , key : & Self :: Key , prev_index : SerializedDepNodeIndex , index : DepNodeIndex ,) -> Option < Self :: Value > ; fn loadable_from_disk (self , qcx : Qcx , key : & Self :: Key , idx : SerializedDepNodeIndex) -> bool ; # [doc = " Synthesize an error value to let compilation continue after a cycle."] fn value_from_cycle_error (self , tcx : Qcx :: DepContext , cycle_error : & CycleError < QueryStackFrameExtra > , guar : ErrorGuaranteed ,) -> Self :: Value ; fn anon (self) -> bool ; fn eval_always (self) -> bool ; fn depth_limit (self) -> bool ; fn feedable (self) -> bool ; fn dep_kind (self) -> DepKind ; fn handle_cycle_error (self) -> HandleCycleError ; fn hash_result (self) -> HashResult < Self :: Value > ; fn construct_dep_node (self , tcx : Qcx :: DepContext , key : & Self :: Key) -> DepNode { DepNode :: construct (tcx , self . dep_kind () , key) } }