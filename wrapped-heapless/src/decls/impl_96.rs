macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < 'a , K , V , const N : usize > OccupiedEntry < 'a , K , V , N > where K : Eq + Hash , { # [doc = " Gets a reference to the key that this entity corresponds to"] pub fn key (& self) -> & K { & self . key } # [doc = " Removes this entry from the map and yields its corresponding key and value"] pub fn remove_entry (self) -> (K , V) { self . core . remove_found (self . probe , self . pos) } # [doc = " Gets a reference to the value associated with this entry"] pub fn get (& self) -> & V { unsafe { & self . core . entries . get_unchecked (self . pos) . value } } # [doc = " Gets a mutable reference to the value associated with this entry"] pub fn get_mut (& mut self) -> & mut V { unsafe { & mut self . core . entries . get_unchecked_mut (self . pos) . value } } # [doc = " Consumes this entry and yields a reference to the underlying value"] pub fn into_mut (self) -> & 'a mut V { unsafe { & mut self . core . entries . get_unchecked_mut (self . pos) . value } } # [doc = " Overwrites the underlying map's value with this entry's value"] pub fn insert (self , value : V) -> V { unsafe { mem :: replace (& mut self . core . entries . get_unchecked_mut (self . pos) . value , value ,) } } # [doc = " Removes this entry from the map and yields its value"] pub fn remove (self) -> V { self . remove_entry () . 1 } }
    };
}

impl_96!()