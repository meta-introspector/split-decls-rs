macro_rules! deps {
    () => {
        RawEntryMut!();
        RawTable!();
        Bucket!();
        HashMap!();
    };
}

macro_rules! RawOccupiedEntryMut {
    () => {
        deps!();
        # [doc = " A view into an occupied entry in a `HashMap`."] # [doc = " It is part of the [`RawEntryMut`] enum."] # [doc = ""] # [doc = " [`RawEntryMut`]: enum.RawEntryMut.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use core::hash::{BuildHasher, Hash};"] # [doc = " use hashbrown::hash_map::{HashMap, RawEntryMut, RawOccupiedEntryMut};"] # [doc = ""] # [doc = " let mut map = HashMap::new();"] # [doc = " map.extend([(\"a\", 10), (\"b\", 20), (\"c\", 30)]);"] # [doc = ""] # [doc = " fn compute_hash<K: Hash + ?Sized, S: BuildHasher>(hash_builder: &S, key: &K) -> u64 {"] # [doc = "     use core::hash::Hasher;"] # [doc = "     let mut state = hash_builder.build_hasher();"] # [doc = "     key.hash(&mut state);"] # [doc = "     state.finish()"] # [doc = " }"] # [doc = ""] # [doc = " let _raw_o: RawOccupiedEntryMut<_, _, _> = map.raw_entry_mut().from_key(&\"a\").insert(\"a\", 100);"] # [doc = " assert_eq!(map.len(), 3);"] # [doc = ""] # [doc = " // Existing key (insert and update)"] # [doc = " match map.raw_entry_mut().from_key(&\"a\") {"] # [doc = "     RawEntryMut::Vacant(_) => unreachable!(),"] # [doc = "     RawEntryMut::Occupied(mut view) => {"] # [doc = "         assert_eq!(view.get(), &100);"] # [doc = "         let v = view.get_mut();"] # [doc = "         let new_v = (*v) * 10;"] # [doc = "         *v = new_v;"] # [doc = "         assert_eq!(view.insert(1111), 1000);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(map[&\"a\"], 1111);"] # [doc = " assert_eq!(map.len(), 3);"] # [doc = ""] # [doc = " // Existing key (take)"] # [doc = " let hash = compute_hash(map.hasher(), &\"c\");"] # [doc = " match map.raw_entry_mut().from_key_hashed_nocheck(hash, &\"c\") {"] # [doc = "     RawEntryMut::Vacant(_) => unreachable!(),"] # [doc = "     RawEntryMut::Occupied(view) => {"] # [doc = "         assert_eq!(view.remove_entry(), (\"c\", 30));"] # [doc = "     }"] # [doc = " }"] # [doc = " assert_eq!(map.raw_entry().from_key(&\"c\"), None);"] # [doc = " assert_eq!(map.len(), 2);"] # [doc = ""] # [doc = " let hash = compute_hash(map.hasher(), &\"b\");"] # [doc = " match map.raw_entry_mut().from_hash(hash, |q| *q == \"b\") {"] # [doc = "     RawEntryMut::Vacant(_) => unreachable!(),"] # [doc = "     RawEntryMut::Occupied(view) => {"] # [doc = "         assert_eq!(view.remove_entry(), (\"b\", 20));"] # [doc = "     }"] # [doc = " }"] # [doc = " assert_eq!(map.get(&\"b\"), None);"] # [doc = " assert_eq!(map.len(), 1);"] # [doc = " ```"] pub struct RawOccupiedEntryMut < 'a , K , V , S , A : Allocator = Global > { elem : Bucket < (K , V) > , table : & 'a mut RawTable < (K , V) , A > , hash_builder : & 'a S , }
    };
}

RawOccupiedEntryMut!();