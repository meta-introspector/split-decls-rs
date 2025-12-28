macro_rules! deps {
    () => {
        Cache!();
        PackCache!();
        ObjectCache!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < S > Cache < S > { # [doc = " Dissolve this instance, discard all caches, and return the inner implementation."] pub fn into_inner (self) -> S { self . inner } # [doc = " Use this methods directly after creating a new instance to add a constructor for pack caches."] # [doc = ""] # [doc = " These are used to speed up decoding objects which are located in packs, reducing long delta chains by storing"] # [doc = " their intermediate results."] pub fn with_pack_cache (mut self , create : impl Fn () -> Box < PackCache > + Send + Sync + 'static) -> Self { self . pack_cache = Some (RefCell :: new (create ())) ; self . new_pack_cache = Some (Arc :: new (create)) ; self } # [doc = " Use this methods directly after creating a new instance to add a constructor for object caches."] # [doc = ""] # [doc = " Only use this kind of cache if the same objects are repeatedly accessed for great speedups, usually during diffing of"] # [doc = " trees."] pub fn with_object_cache (mut self , create : impl Fn () -> Box < ObjectCache > + Send + Sync + 'static) -> Self { self . object_cache = Some (RefCell :: new (create ())) ; self . new_object_cache = Some (Arc :: new (create)) ; self } # [doc = " Set the pack cache constructor on this instance."] pub fn set_pack_cache (& mut self , create : impl Fn () -> Box < PackCache > + Send + Sync + 'static) { self . pack_cache = Some (RefCell :: new (create ())) ; self . new_pack_cache = Some (Arc :: new (create)) ; } # [doc = " Set the object cache constructor on this instance."] pub fn set_object_cache (& mut self , create : impl Fn () -> Box < ObjectCache > + Send + Sync + 'static) { self . object_cache = Some (RefCell :: new (create ())) ; self . new_object_cache = Some (Arc :: new (create)) ; } # [doc = " Return true if an object cache is present."] pub fn has_object_cache (& self) -> bool { self . object_cache . is_some () } # [doc = " Return true if a pack cache is present."] pub fn has_pack_cache (& self) -> bool { self . pack_cache . is_some () } # [doc = " Remove the current pack cache as well as its constructor from this instance."] pub fn unset_pack_cache (& mut self) { self . pack_cache = None ; self . new_pack_cache = None ; } # [doc = " Remove the current object cache as well as its constructor from this instance."] pub fn unset_object_cache (& mut self) { self . object_cache = None ; self . new_object_cache = None ; } }
    };
}

impl_138!();