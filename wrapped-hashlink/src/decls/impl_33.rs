macro_rules! deps {
    () => {
        RawEntryBuilder!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , K , V , S > RawEntryBuilder < 'a , K , V , S > where S : BuildHasher , { # [inline] pub fn from_key < Q > (self , k : & Q) -> Option < (& 'a K , & 'a V) > where K : Borrow < Q > , Q : Hash + Eq + ? Sized , { let hash = hash_key (& self . map . hash_builder , k) ; self . from_key_hashed_nocheck (hash , k) } # [inline] pub fn from_key_hashed_nocheck < Q > (self , hash : u64 , k : & Q) -> Option < (& 'a K , & 'a V) > where K : Borrow < Q > , Q : Hash + Eq + ? Sized , { self . from_hash (hash , move | o | k . eq (o . borrow ())) } # [inline] pub fn from_hash (self , hash : u64 , mut is_match : impl FnMut (& K) -> bool ,) -> Option < (& 'a K , & 'a V) > { unsafe { let node = self . map . table . find (hash , move | k | is_match ((* k) . as_ref () . key_ref ())) ? ; let (key , value) = (* node . as_ptr ()) . entry_ref () ; Some ((key , value)) } } }
    };
}

impl_33!();