macro_rules! deps {
    () => {
        CursorMut!();
        OccupiedEntry!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'a , K , V , S > OccupiedEntry < 'a , K , V , S > { # [inline] pub fn key (& self) -> & K { self . raw_entry . key () } # [inline] pub fn remove_entry (self) -> (K , V) { self . raw_entry . remove_entry () } # [inline] pub fn get (& self) -> & V { self . raw_entry . get () } # [inline] pub fn get_mut (& mut self) -> & mut V { self . raw_entry . get_mut () } # [inline] pub fn into_mut (self) -> & 'a mut V { self . raw_entry . into_mut () } # [inline] pub fn to_back (& mut self) { self . raw_entry . to_back () } # [inline] pub fn to_front (& mut self) { self . raw_entry . to_front () } # [doc = " Replaces this entry's value with the provided value."] # [doc = ""] # [doc = " Similarly to `LinkedHashMap::insert`, this moves the existing entry to the back of the"] # [doc = " internal linked list."] # [inline] pub fn insert (& mut self , value : V) -> V { self . raw_entry . to_back () ; self . raw_entry . replace_value (value) } # [inline] pub fn remove (self) -> V { self . raw_entry . remove () } # [doc = " Similar to `OccupiedEntry::replace_entry`, but *does* move the entry to the back of the"] # [doc = " internal linked list."] # [inline] pub fn insert_entry (mut self , value : V) -> (K , V) { self . raw_entry . to_back () ; self . replace_entry (value) } # [doc = " Returns a `CursorMut` over the current entry."] # [inline] pub fn cursor_mut (self) -> CursorMut < 'a , K , V , S > where K : Eq + Hash , S : BuildHasher , { self . raw_entry . cursor_mut () } # [doc = " Replaces the entry's key with the key provided to `LinkedHashMap::entry`, and replaces the"] # [doc = " entry's value with the given `value` parameter."] # [doc = ""] # [doc = " Does *not* move the entry to the back of the internal linked list."] pub fn replace_entry (mut self , value : V) -> (K , V) { let old_key = mem :: replace (self . raw_entry . key_mut () , self . key) ; let old_value = mem :: replace (self . raw_entry . get_mut () , value) ; (old_key , old_value) } # [doc = " Replaces this entry's key with the key provided to `LinkedHashMap::entry`."] # [doc = ""] # [doc = " Does *not* move the entry to the back of the internal linked list."] # [inline] pub fn replace_key (mut self) -> K { mem :: replace (self . raw_entry . key_mut () , self . key) } }
    };
}

impl_28!()