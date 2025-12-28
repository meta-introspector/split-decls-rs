macro_rules! deps {
    () => {
        Note!();
        State!();
        Repository!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        # [doc = " Configure how caches are used to speed up various git repository operations"] impl crate :: Repository { # [doc = " Sets the amount of space used at most for caching most recently accessed fully decoded objects, to `Some(bytes)`,"] # [doc = " or `None` to deactivate it entirely."] # [doc = ""] # [doc = " Note that it is unset by default but can be enabled once there is time for performance optimization."] # [doc = " Well-chosen cache sizes can improve performance particularly if objects are accessed multiple times in a row."] # [doc = " The cache is configured to grow gradually."] # [doc = ""] # [doc = " Note that a cache on application level should be considered as well as the best object access is not doing one."] pub fn object_cache_size (& mut self , bytes : impl Into < Option < usize > >) { let bytes = bytes . into () ; match bytes { Some (0) => self . objects . unset_object_cache () , Some (bytes) => self . objects . set_object_cache (move | | Box :: new (crate :: object :: cache :: MemoryCappedHashmap :: new (bytes))) , None => self . objects . unset_object_cache () , } } # [doc = " Set an object cache of size `bytes` if none is set."] # [doc = ""] # [doc = " Use this method to avoid overwriting any existing value while assuring better performance in case no value is set."] pub fn object_cache_size_if_unset (& mut self , bytes : usize) { if ! self . objects . has_object_cache () { self . object_cache_size (bytes) ; } } # [doc = " Return the amount of bytes the object cache [should be set to](Self::object_cache_size_if_unset) to perform"] # [doc = " diffs between trees who are similar to `index` in a typical source code repository."] # [doc = ""] # [doc = " Currently, this allocates about 10MB for every 10k files in `index`, and a minimum of 4KB."] # [cfg (feature = "index")] pub fn compute_object_cache_size_for_tree_diffs (& self , index : & gix_index :: State) -> usize { let num_tracked = index . entries () . len () ; let ten_mb_for_every_10k_files = (num_tracked as f32 / 10_000.0) * (10 * 1024 * 1024) as f32 ; (ten_mb_for_every_10k_files as usize) . max (4 * 1024) } }
    };
}

impl_292!()