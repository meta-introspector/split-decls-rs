macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'a , K , V > OccupiedEntry < 'a , K , V > where K : Eq , { # [doc = " Gets a reference to the key that this entity corresponds to"] pub fn key (& self) -> & K { let (k , _v) = unsafe { self . map . buffer . get_unchecked (self . idx) } ; k } # [doc = " Removes this entry from the map and yields its corresponding key and value"] pub fn remove_entry (self) -> (K , V) { unsafe { self . map . buffer . swap_remove_unchecked (self . idx) } } # [doc = " Removes this entry from the map and yields its corresponding key and value"] pub fn remove (self) -> V { self . remove_entry () . 1 } # [doc = " Gets a reference to the value associated with this entry"] pub fn get (& self) -> & V { let (_k , v) = unsafe { self . map . buffer . get_unchecked (self . idx) } ; v } # [doc = " Gets a mutable reference to the value associated with this entry"] pub fn get_mut (& mut self) -> & mut V { let (_k , v) = unsafe { self . map . buffer . get_unchecked_mut (self . idx) } ; v } # [doc = " Consumes this entry and yields a reference to the underlying value"] pub fn into_mut (self) -> & 'a mut V { let (_k , v) = unsafe { self . map . buffer . get_unchecked_mut (self . idx) } ; v } # [doc = " Overwrites the underlying map's value with this entry's value"] pub fn insert (self , value : V) -> V { let (_k , v) = unsafe { self . map . buffer . get_unchecked_mut (self . idx) } ; mem :: replace (v , value) } }
    };
}

impl_191!()