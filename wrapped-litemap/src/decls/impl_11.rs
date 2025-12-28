macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < K : ? Sized , V : ? Sized , S > LiteMap < K , V , S > where K : Ord , S : Store < K , V > , { # [doc = " Get the value associated with `key`, if it exists."] # [doc = ""] # [doc = " ```rust"] # [doc = " use litemap::LiteMap;"] # [doc = ""] # [doc = " let mut map = LiteMap::new_vec();"] # [doc = " map.insert(1, \"one\");"] # [doc = " map.insert(2, \"two\");"] # [doc = " assert_eq!(map.get(&1), Some(&\"one\"));"] # [doc = " assert_eq!(map.get(&3), None);"] # [doc = " ```"] pub fn get < Q > (& self , key : & Q) -> Option < & V > where K : Borrow < Q > , Q : Ord + ? Sized , { match self . find_index (key) { # [expect (clippy :: unwrap_used)] Ok (found) => Some (self . values . lm_get (found) . unwrap () . 1) , Err (_) => None , } } # [doc = " Binary search the map with `predicate` to find a key, returning the value."] pub fn get_by (& self , predicate : impl FnMut (& K) -> Ordering) -> Option < & V > { let index = self . values . lm_binary_search_by (predicate) . ok () ? ; self . values . lm_get (index) . map (| (_ , v) | v) } # [doc = " Returns whether `key` is contained in this map"] # [doc = ""] # [doc = " ```rust"] # [doc = " use litemap::LiteMap;"] # [doc = ""] # [doc = " let mut map = LiteMap::new_vec();"] # [doc = " map.insert(1, \"one\");"] # [doc = " map.insert(2, \"two\");"] # [doc = " assert!(map.contains_key(&1));"] # [doc = " assert!(!map.contains_key(&3));"] # [doc = " ```"] pub fn contains_key < Q > (& self , key : & Q) -> bool where K : Borrow < Q > , Q : Ord + ? Sized , { self . find_index (key) . is_ok () } # [doc = " Obtain the index for a given key, or if the key is not found, the index"] # [doc = " at which it would be inserted."] # [doc = ""] # [doc = " (The return value works equivalently to [`slice::binary_search_by()`])"] # [doc = ""] # [doc = " The indices returned can be used with [`Self::get_indexed()`]. Prefer using"] # [doc = " [`Self::get()`] directly where possible."] # [inline] pub fn find_index < Q > (& self , key : & Q) -> Result < usize , usize > where K : Borrow < Q > , Q : Ord + ? Sized , { self . values . lm_binary_search_by (| k | k . borrow () . cmp (key)) } }
    };
}

impl_11!();