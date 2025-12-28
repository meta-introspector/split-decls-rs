macro_rules! deps {
    () => {
        OccupiedEntry!();
        VacantEntry!();
        HashSet!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl < 'a , T , S , A : Allocator > VacantEntry < 'a , T , S , A > { # [doc = " Gets a reference to the value that would be used when inserting"] # [doc = " through the `VacantEntry`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = ""] # [doc = " let mut set: HashSet<&str> = HashSet::new();"] # [doc = " assert_eq!(set.entry(\"poneyland\").get(), &\"poneyland\");"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn get (& self) -> & T { self . inner . key () } # [doc = " Take ownership of the value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::hash_set::{Entry, HashSet};"] # [doc = ""] # [doc = " let mut set: HashSet<&str> = HashSet::new();"] # [doc = ""] # [doc = " match set.entry(\"poneyland\") {"] # [doc = "     Entry::Occupied(_) => panic!(),"] # [doc = "     Entry::Vacant(v) => assert_eq!(v.into_value(), \"poneyland\"),"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn into_value (self) -> T { self . inner . into_key () } # [doc = " Sets the value of the entry with the `VacantEntry`'s value."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashSet;"] # [doc = " use hashbrown::hash_set::Entry;"] # [doc = ""] # [doc = " let mut set: HashSet<&str> = HashSet::new();"] # [doc = ""] # [doc = " if let Entry::Vacant(o) = set.entry(\"poneyland\") {"] # [doc = "     o.insert();"] # [doc = " }"] # [doc = " assert!(set.contains(\"poneyland\"));"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn insert (self) -> OccupiedEntry < 'a , T , S , A > where T : Hash , S : BuildHasher , { OccupiedEntry { inner : self . inner . insert_entry (()) , } } }
    };
}

impl_464!()