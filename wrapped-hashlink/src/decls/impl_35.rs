macro_rules! deps {
    () => {
        RawEntryMut!();
        RawEntryBuilderMut!();
        RawOccupiedEntryMut!();
        RawVacantEntryMut!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a , K , V , S > RawEntryBuilderMut < 'a , K , V , S > where S : BuildHasher , { # [inline] pub fn from_key < Q > (self , k : & Q) -> RawEntryMut < 'a , K , V , S > where K : Borrow < Q > , Q : Hash + Eq + ? Sized , { let hash = hash_key (& self . map . hash_builder , k) ; self . from_key_hashed_nocheck (hash , k) } # [inline] pub fn from_key_hashed_nocheck < Q > (self , hash : u64 , k : & Q) -> RawEntryMut < 'a , K , V , S > where K : Borrow < Q > , Q : Hash + Eq + ? Sized , { self . from_hash (hash , move | o | k . eq (o . borrow ())) } # [inline] pub fn from_hash (self , hash : u64 , mut is_match : impl FnMut (& K) -> bool ,) -> RawEntryMut < 'a , K , V , S > { let entry = self . map . table . find_entry (hash , move | k | is_match (unsafe { (* k) . as_ref () . key_ref () })) ; match entry { Ok (occupied) => RawEntryMut :: Occupied (RawOccupiedEntryMut { hash_builder : & self . map . hash_builder , free : & mut self . map . free , values : & mut self . map . values , entry : occupied , }) , Err (absent) => RawEntryMut :: Vacant (RawVacantEntryMut { hash_builder : & self . map . hash_builder , values : & mut self . map . values , free : & mut self . map . free , entry : absent , }) , } } }
    };
}

impl_35!()