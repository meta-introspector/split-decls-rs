/* FP:fx.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_USE_0001
/* FP:fx.rs-0002 */ use std :: hash :: BuildHasherDefault ;
/* FP:fx.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_USE_0002
/* FP:fx.rs-0004 */ pub use crate :: rustc_hash :: { FxHashMap , FxHashSet , FxHasher } ;
/* FP:fx.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_TYPE_0003
/* FP:fx.rs-0006 */ pub type StdEntry < 'a , K , V > = std :: collections :: hash_map :: Entry < 'a , K , V > ;
/* FP:fx.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_TYPE_0004
/* FP:fx.rs-0008 */ pub type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < FxHasher > > ;
/* FP:fx.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_TYPE_0005
/* FP:fx.rs-0010 */ pub type FxIndexSet < V > = indexmap :: IndexSet < V , BuildHasherDefault < FxHasher > > ;
/* FP:fx.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_TYPE_0006
/* FP:fx.rs-0012 */ pub type IndexEntry < 'a , K , V > = indexmap :: map :: Entry < 'a , K , V > ;
/* FP:fx.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_TYPE_0007
/* FP:fx.rs-0014 */ pub type IndexOccupiedEntry < 'a , K , V > = indexmap :: map :: OccupiedEntry < 'a , K , V > ;
/* FP:fx.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_USE_0008
/* FP:fx.rs-0016 */ pub use indexmap :: set :: MutableValues ;
/* FP:fx.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_MACRO_0009
/* FP:fx.rs-0018 */ # [macro_export] macro_rules ! define_id_collections { ($ map_name : ident , $ set_name : ident , $ entry_name : ident , $ key : ty) => { pub type $ map_name < T > = $ crate :: unord :: UnordMap <$ key , T >; pub type $ set_name = $ crate :: unord :: UnordSet <$ key >; pub type $ entry_name <'a , T > = $ crate :: fx :: StdEntry <'a , $ key , T >; } ; }
/* FP:fx.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_fx_MACRO_0010
/* FP:fx.rs-0020 */ # [macro_export] macro_rules ! define_stable_id_collections { ($ map_name : ident , $ set_name : ident , $ entry_name : ident , $ key : ty) => { pub type $ map_name < T > = $ crate :: fx :: FxIndexMap <$ key , T >; pub type $ set_name = $ crate :: fx :: FxIndexSet <$ key >; pub type $ entry_name <'a , T > = $ crate :: fx :: IndexEntry <'a , $ key , T >; } ; }