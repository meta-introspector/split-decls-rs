macro_rules! deps {
    () => {
        RawEntryMut!();
        RawEntryBuilderMut!();
        Equivalent!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl < 'a , K , V , S , A : Allocator > RawEntryBuilderMut < 'a , K , V , S , A > { # [doc = " Creates a `RawEntryMut` from the given key."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_map::{HashMap, RawEntryMut};"] # [doc = ""] # [doc = " let mut map: HashMap<&str, u32> = HashMap::new();"] # [doc = " let key = \"a\";"] # [doc = " let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key(&key);"] # [doc = " entry.insert(key, 100);"] # [doc = " assert_eq!(map[&\"a\"], 100);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] # [allow (clippy :: wrong_self_convention)] pub fn from_key < Q > (self , k : & Q) -> RawEntryMut < 'a , K , V , S , A > where S : BuildHasher , Q : Hash + Equivalent < K > + ? Sized , { let hash = make_hash :: < Q , S > (& self . map . hash_builder , k) ; self . from_key_hashed_nocheck (hash , k) } # [doc = " Creates a `RawEntryMut` from the given key and its hash."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::hash::{BuildHasher, Hash};"] # [doc = " use hashbrown::hash_map::{HashMap, RawEntryMut};"] # [doc = ""] # [doc = " fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {"] # [doc = "     use core::hash::Hasher;"] # [doc = "     let mut state = hash_builder.build_hasher();"] # [doc = "     key.hash(&mut state);"] # [doc = "     state.finish()"] # [doc = " }"] # [doc = ""] # [doc = " let mut map: HashMap<&str, u32> = HashMap::new();"] # [doc = " let key = \"a\";"] # [doc = " let hash = compute_hash(map.hasher(), &key);"] # [doc = " let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_key_hashed_nocheck(hash, &key);"] # [doc = " entry.insert(key, 100);"] # [doc = " assert_eq!(map[&\"a\"], 100);"] # [doc = " ```"] # [inline] # [allow (clippy :: wrong_self_convention)] pub fn from_key_hashed_nocheck < Q > (self , hash : u64 , k : & Q) -> RawEntryMut < 'a , K , V , S , A > where Q : Equivalent < K > + ? Sized , { self . from_hash (hash , equivalent (k)) } }
    };
}

impl_351!()