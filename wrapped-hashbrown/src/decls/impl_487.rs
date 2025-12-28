macro_rules! deps {
    () => {
        HashTable!();
        VacantEntry!();
        DefaultHashBuilder!();
        OccupiedEntry!();
    };
}

macro_rules! impl_487 {
    () => {
        deps!();
        impl < 'a , T , A > VacantEntry < 'a , T , A > where A : Allocator , { # [doc = " Inserts a new element into the table with the hash that was used to"] # [doc = " obtain the `VacantEntry`."] # [doc = ""] # [doc = " An `OccupiedEntry` is returned for the newly inserted element."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"nightly\")]"] # [doc = " # fn test() {"] # [doc = " use hashbrown::hash_table::Entry;"] # [doc = " use hashbrown::{HashTable, DefaultHashBuilder};"] # [doc = " use std::hash::BuildHasher;"] # [doc = ""] # [doc = " let mut table: HashTable<&str> = HashTable::new();"] # [doc = " let hasher = DefaultHashBuilder::default();"] # [doc = " let hasher = |val: &_| hasher.hash_one(val);"] # [doc = ""] # [doc = " if let Entry::Vacant(o) = table.entry(hasher(&\"poneyland\"), |&x| x == \"poneyland\", hasher) {"] # [doc = "     o.insert(\"poneyland\");"] # [doc = " }"] # [doc = " assert_eq!("] # [doc = "     table.find(hasher(&\"poneyland\"), |&x| x == \"poneyland\"),"] # [doc = "     Some(&\"poneyland\")"] # [doc = " );"] # [doc = " # }"] # [doc = " # fn main() {"] # [doc = " #     #[cfg(feature = \"nightly\")]"] # [doc = " #     test()"] # [doc = " # }"] # [doc = " ```"] # [inline] pub fn insert (self , value : T) -> OccupiedEntry < 'a , T , A > { let bucket = unsafe { self . table . raw . insert_at_index (self . hash , self . index , value) } ; OccupiedEntry { hash : self . hash , bucket , table : self . table , } } # [doc = " Converts the `VacantEntry` into a mutable reference to the underlying"] # [doc = " table."] pub fn into_table (self) -> & 'a mut HashTable < T , A > { self . table } }
    };
}

impl_487!();