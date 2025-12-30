// Generated macro for impl_972 (impl)
macro_rules! Depcrate_token_memory_cacheimpl_972 {
() => {
// Module: crate::token_memory_cache
// Provides: {"impl_972"}
// Dependencies: {}
impl State { fn new (max_server_names : u32 , max_tokens_per_server : usize) -> Self { Self { max_server_names , max_tokens_per_server , lookup : HashMap :: new () , lru : LruSlab :: default () , } } fn store (& mut self , server_name : & str , token : Bytes) { if self . max_server_names == 0 { return ; } if self . max_tokens_per_server == 0 { return ; } let server_name = Arc :: < str > :: from (server_name) ; match self . lookup . entry (server_name . clone ()) { hash_map :: Entry :: Occupied (hmap_entry) => { let tokens = & mut self . lru . get_mut (* hmap_entry . get ()) . tokens ; if tokens . len () >= self . max_tokens_per_server { debug_assert ! (tokens . len () == self . max_tokens_per_server) ; tokens . pop_front () . unwrap () ; } tokens . push_back (token) ; } hash_map :: Entry :: Vacant (hmap_entry) => { let removed_key = if self . lru . len () >= self . max_server_names { Some (self . lru . remove (self . lru . lru () . unwrap ()) . server_name) } else { None } ; hmap_entry . insert (self . lru . insert (CacheEntry :: new (server_name , token))) ; if let Some (removed_slot) = removed_key { let removed = self . lookup . remove (& removed_slot) ; debug_assert ! (removed . is_some ()) ; } } } ; } fn take (& mut self , server_name : & str) -> Option < Bytes > { let slab_key = * self . lookup . get (server_name) ? ; let entry = self . lru . get_mut (slab_key) ; let token = entry . tokens . pop_front () . unwrap () ; if entry . tokens . is_empty () { self . lru . remove (slab_key) ; self . lookup . remove (server_name) ; } Some (token) } }
};
}
