// Generated macro for impl_216 (impl)
macro_rules! Depcrate_cacheimpl_216 {
() => {
// Module: crate::cache
// Provides: {"impl_216"}
// Dependencies: {}
impl Cache < crate :: store :: Handle < Rc < crate :: Store > > > { # [doc = " Convert this cache's handle into one that keeps its store in an arc. This creates an entirely new store,"] # [doc = " so should be done early to avoid unnecessary work (and mappings)."] pub fn into_arc (self) -> std :: io :: Result < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > { let inner = self . inner . into_arc () ? ; Ok (Cache { inner , new_pack_cache : self . new_pack_cache , new_object_cache : self . new_object_cache , pack_cache : self . pack_cache , object_cache : self . object_cache , }) } }
};
}
