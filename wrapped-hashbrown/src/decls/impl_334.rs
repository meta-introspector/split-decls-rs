macro_rules! deps {
    () => {
        VacantEntryRef!();
        OccupiedEntry!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < 'a , 'b , K , Q : ? Sized , V , S , A : Allocator > VacantEntryRef < 'a , 'b , K , Q , V , S , A > { # [doc = " Gets a reference to the key that would be used when inserting a value"] # [doc = " through the `VacantEntryRef`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<String, u32> = HashMap::new();"] # [doc = " let key: &str = \"poneyland\";"] # [doc = " assert_eq!(map.entry_ref(key).key(), \"poneyland\");"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn key (& self) -> & 'b Q { self . key } # [doc = " Sets the value of the entry with the `VacantEntryRef`'s key,"] # [doc = " and returns a mutable reference to it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = " use hashbrown::hash_map::EntryRef;"] # [doc = ""] # [doc = " let mut map: HashMap<String, u32> = HashMap::new();"] # [doc = " let key: &str = \"poneyland\";"] # [doc = ""] # [doc = " if let EntryRef::Vacant(o) = map.entry_ref(key) {"] # [doc = "     o.insert(37);"] # [doc = " }"] # [doc = " assert_eq!(map[\"poneyland\"], 37);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn insert (self , value : V) -> & 'a mut V where K : Hash , & 'b Q : Into < K > , S : BuildHasher , { let table = & mut self . table . table ; let entry = table . insert_entry (self . hash , (self . key . into () , value) , make_hasher :: < _ , V , S > (& self . table . hash_builder) ,) ; & mut entry . 1 } # [doc = " Sets the value of the entry with the [`VacantEntryRef`]'s key,"] # [doc = " and returns an [`OccupiedEntry`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = " use hashbrown::hash_map::EntryRef;"] # [doc = ""] # [doc = " let mut map: HashMap<&str, u32> = HashMap::new();"] # [doc = ""] # [doc = " if let EntryRef::Vacant(v) = map.entry_ref(\"poneyland\") {"] # [doc = "     let o = v.insert_entry(37);"] # [doc = "     assert_eq!(o.get(), &37);"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn insert_entry (self , value : V) -> OccupiedEntry < 'a , K , V , S , A > where K : Hash , & 'b Q : Into < K > , S : BuildHasher , { let elem = self . table . table . insert (self . hash , (self . key . into () , value) , make_hasher :: < _ , V , S > (& self . table . hash_builder) ,) ; OccupiedEntry { hash : self . hash , elem , table : self . table , } } }
    };
}

impl_334!();