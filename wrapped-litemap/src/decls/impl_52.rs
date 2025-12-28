macro_rules! deps {
    () => {
        VacantEntry!();
        StoreMut!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < 'a , K , V , S > VacantEntry < 'a , K , V , S > where K : Ord , S : StoreMut < K , V > , { # [doc = " Gets a reference to the key that would be used when inserting a value through the `VacantEntry`."] pub fn key (& self) -> & K { & self . key } # [doc = " Sets the value of the entry with the `VacantEntry`'s key, and returns a mutable reference to it."] pub fn insert (self , value : V) -> & 'a mut V { self . map . values . lm_insert (self . index , self . key , value) ; # [expect (clippy :: unwrap_used)] self . map . values . lm_get_mut (self . index) . unwrap () . 1 } }
    };
}

impl_52!()