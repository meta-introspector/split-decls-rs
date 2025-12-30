// Generated macro for OccupiedError (struct)
macro_rules! Depcrate_mapOccupiedError {
() => {
// Module: crate::map
// Provides: {"OccupiedError"}
// Dependencies: {}
# [doc = " The error returned by [`try_insert`](HashMap::try_insert) when the key already exists."] # [doc = ""] # [doc = " Contains the occupied entry, and the value that was not inserted."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_map::{HashMap, OccupiedError};"] # [doc = ""] # [doc = " let mut map: HashMap<_, _> = [(\"a\", 10), (\"b\", 20)].into();"] # [doc = ""] # [doc = " // try_insert method returns mutable reference to the value if keys are vacant,"] # [doc = " // but if the map did have key present, nothing is updated, and the provided"] # [doc = " // value is returned inside `Err(_)` variant"] # [doc = " match map.try_insert(\"a\", 100) {"] # [doc = "     Err(OccupiedError { mut entry, value }) => {"] # [doc = "         assert_eq!(entry.key(), &\"a\");"] # [doc = "         assert_eq!(value, 100);"] # [doc = "         assert_eq!(entry.insert(100), 10)"] # [doc = "     }"] # [doc = "     _ => unreachable!(),"] # [doc = " }"] # [doc = " assert_eq!(map[&\"a\"], 100);"] # [doc = " ```"] pub struct OccupiedError < 'a , K , V , S , A : Allocator = Global > { # [doc = " The entry in the map that was already occupied."] pub entry : OccupiedEntry < 'a , K , V , S , A > , # [doc = " The value which was not inserted, because the entry was already occupied."] pub value : V , }
};
}
