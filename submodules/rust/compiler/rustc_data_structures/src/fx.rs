mkuse!{use std :: hash :: BuildHasherDefault ;}
mkuse!{pub use rustc_hash :: { FxHashMap , FxHashSet , FxHasher } ;}
mkitem!{pub type StdEntry < 'a , K , V > = std :: collections :: hash_map :: Entry < 'a , K , V > ;}
mkitem!{pub type FxIndexMap < K , V > = indexmap :: IndexMap < K , V , BuildHasherDefault < FxHasher > > ;}
mkitem!{pub type FxIndexSet < V > = indexmap :: IndexSet < V , BuildHasherDefault < FxHasher > > ;}
mkitem!{pub type IndexEntry < 'a , K , V > = indexmap :: map :: Entry < 'a , K , V > ;}
mkitem!{pub type IndexOccupiedEntry < 'a , K , V > = indexmap :: map :: OccupiedEntry < 'a , K , V > ;}
mkuse!{pub use indexmap :: set :: MutableValues ;}
mkitem!{# [macro_export] macro_rules ! define_id_collections { ($ map_name : ident , $ set_name : ident , $ entry_name : ident , $ key : ty) => { pub type $ map_name < T > = $ crate :: unord :: UnordMap <$ key , T >; pub type $ set_name = $ crate :: unord :: UnordSet <$ key >; pub type $ entry_name <'a , T > = $ crate :: fx :: StdEntry <'a , $ key , T >; } ; }}
mkitem!{# [macro_export] macro_rules ! define_stable_id_collections { ($ map_name : ident , $ set_name : ident , $ entry_name : ident , $ key : ty) => { pub type $ map_name < T > = $ crate :: fx :: FxIndexMap <$ key , T >; pub type $ set_name = $ crate :: fx :: FxIndexSet <$ key >; pub type $ entry_name <'a , T > = $ crate :: fx :: IndexEntry <'a , $ key , T >; } ; }}