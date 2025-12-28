macro_rules! deps {
    () => {
        OccupiedEntry!();
        StoreMut!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'a , K , V , S > OccupiedEntry < 'a , K , V , S > where K : Ord , S : StoreMut < K , V > , { # [doc = " Gets a reference to the key in the entry."] pub fn key (& self) -> & K { # [expect (clippy :: unwrap_used)] self . map . values . lm_get (self . index) . unwrap () . 0 } # [doc = " Gets a reference to the value in the entry."] pub fn get (& self) -> & V { # [expect (clippy :: unwrap_used)] self . map . values . lm_get (self . index) . unwrap () . 1 } # [doc = " Gets a mutable reference to the value in the entry."] pub fn get_mut (& mut self) -> & mut V { # [expect (clippy :: unwrap_used)] self . map . values . lm_get_mut (self . index) . unwrap () . 1 } # [doc = " Converts the entry into a mutable reference to the value in the entry with a lifetime bound to the map."] pub fn into_mut (self) -> & 'a mut V { # [expect (clippy :: unwrap_used)] self . map . values . lm_get_mut (self . index) . unwrap () . 1 } # [doc = " Sets the value of the entry, and returns the entry's old value."] pub fn insert (& mut self , value : V) -> V { mem :: replace (self . get_mut () , value) } # [doc = " Takes the value out of the entry, and returns it."] pub fn remove (self) -> V { self . map . values . lm_remove (self . index) . 1 } }
    };
}

impl_51!();