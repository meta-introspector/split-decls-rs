macro_rules! deps {
    () => {
        Cache!();
        OdbHandle!();
    };
}

macro_rules! setup_objects {
    () => {
        deps!();
        # [cfg_attr (not (feature = "max-performance-safe") , allow (unused_variables , unused_mut))] pub (crate) fn setup_objects (objects : & mut crate :: OdbHandle , config : & crate :: config :: Cache) { # [cfg (feature = "max-performance-safe")] { match config . pack_cache_bytes { None => match config . static_pack_cache_limit_bytes { None => objects . set_pack_cache (| | Box :: < gix_pack :: cache :: lru :: StaticLinkedList < 64 > > :: default ()) , Some (limit) => { objects . set_pack_cache (move | | Box :: new (gix_pack :: cache :: lru :: StaticLinkedList :: < 64 > :: new (limit))) ; } } , Some (0) => objects . unset_pack_cache () , Some (bytes) => objects . set_pack_cache (move | | -> Box < gix_odb :: cache :: PackCache > { Box :: new (gix_pack :: cache :: lru :: MemoryCappedHashmap :: new (bytes)) }) , } if config . object_cache_bytes == 0 { objects . unset_object_cache () ; } else { let bytes = config . object_cache_bytes ; objects . set_object_cache (move | | Box :: new (gix_pack :: cache :: object :: MemoryCappedHashmap :: new (bytes))) ; } } }
    };
}

setup_objects!();