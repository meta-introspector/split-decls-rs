// Generated macro for impl_357 (impl)
macro_rules! Depcrateimpl_357 {
() => {
// Module: crate
// Provides: {"impl_357"}
// Dependencies: {}
impl RootDatabase { pub fn new (lru_capacity : Option < u16 >) -> RootDatabase { let mut db = RootDatabase { storage : ManuallyDrop :: new (salsa :: Storage :: default ()) , files : Default :: default () , crates_map : Default :: default () , } ; db . set_all_crates (Arc :: new (Box :: new ([]))) ; CrateGraphBuilder :: default () . set_in_db (& mut db) ; db . set_proc_macros_with_durability (Default :: default () , Durability :: MEDIUM) ; db . set_local_roots_with_durability (Default :: default () , Durability :: MEDIUM) ; db . set_library_roots_with_durability (Default :: default () , Durability :: MEDIUM) ; db . set_expand_proc_attr_macros_with_durability (false , Durability :: HIGH) ; db . update_base_query_lru_capacities (lru_capacity) ; db } pub fn enable_proc_attr_macros (& mut self) { self . set_expand_proc_attr_macros_with_durability (true , Durability :: HIGH) ; } pub fn update_base_query_lru_capacities (& mut self , _lru_capacity : Option < u16 >) { } pub fn update_lru_capacities (& mut self , _lru_capacities : & FxHashMap < Box < str > , u16 >) { } }
};
}
