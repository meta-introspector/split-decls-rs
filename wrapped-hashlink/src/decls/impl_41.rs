macro_rules! deps {
    () => {
        RawVacantEntryMut!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl < 'a , K , V , S > RawVacantEntryMut < 'a , K , V , S > { # [inline] pub fn insert (self , key : K , value : V) -> (& 'a mut K , & 'a mut V) where K : Hash , S : BuildHasher , { let hash = hash_key (self . hash_builder , & key) ; self . insert_hashed_nocheck (hash , key , value) } # [inline] pub fn insert_hashed_nocheck (self , hash : u64 , key : K , value : V) -> (& 'a mut K , & 'a mut V) where K : Hash , S : BuildHasher , { let hash_builder = self . hash_builder ; self . insert_with_hasher (hash , key , value , | k | hash_key (hash_builder , k)) } # [inline] pub fn insert_with_hasher (self , hash : u64 , key : K , value : V , hasher : impl Fn (& K) -> u64 ,) -> (& 'a mut K , & 'a mut V) where S : BuildHasher , { unsafe { ensure_guard_node (self . values) ; let mut new_node = allocate_node (self . free) ; new_node . as_mut () . put_entry ((key , value)) ; attach_before (new_node , NonNull :: new_unchecked (self . values . as_ptr ())) ; let node = self . entry . into_table () . insert_unique (hash , new_node , move | k | hasher ((* k) . as_ref () . key_ref ())) . into_mut () ; let (key , value) = (* node . as_ptr ()) . entry_mut () ; (key , value) } } }
    };
}

impl_41!()