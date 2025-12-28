macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
        RawEntryMut!();
        RawVacantEntryMut!();
        RawEntryBuilderMut!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl < 'a , K , V , S , A : Allocator > RawEntryBuilderMut < 'a , K , V , S , A > { # [doc = " Creates a `RawEntryMut` from the given hash and matching function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::hash::{BuildHasher, Hash};"] # [doc = " use hashbrown::hash_map::{HashMap, RawEntryMut};"] # [doc = ""] # [doc = " fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {"] # [doc = "     use core::hash::Hasher;"] # [doc = "     let mut state = hash_builder.build_hasher();"] # [doc = "     key.hash(&mut state);"] # [doc = "     state.finish()"] # [doc = " }"] # [doc = ""] # [doc = " let mut map: HashMap<&str, u32> = HashMap::new();"] # [doc = " let key = \"a\";"] # [doc = " let hash = compute_hash(map.hasher(), &key);"] # [doc = " let entry: RawEntryMut<&str, u32, _> = map.raw_entry_mut().from_hash(hash, |k| k == &key);"] # [doc = " entry.insert(key, 100);"] # [doc = " assert_eq!(map[&\"a\"], 100);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] # [allow (clippy :: wrong_self_convention)] pub fn from_hash < F > (self , hash : u64 , is_match : F) -> RawEntryMut < 'a , K , V , S , A > where for < 'b > F : FnMut (& 'b K) -> bool , { self . search (hash , is_match) } # [cfg_attr (feature = "inline-more" , inline)] fn search < F > (self , hash : u64 , mut is_match : F) -> RawEntryMut < 'a , K , V , S , A > where for < 'b > F : FnMut (& 'b K) -> bool , { match self . map . table . find (hash , | (k , _) | is_match (k)) { Some (elem) => RawEntryMut :: Occupied (RawOccupiedEntryMut { elem , table : & mut self . map . table , hash_builder : & self . map . hash_builder , }) , None => RawEntryMut :: Vacant (RawVacantEntryMut { table : & mut self . map . table , hash_builder : & self . map . hash_builder , }) , } } }
    };
}

impl_352!()