macro_rules! deps {
    () => {
        Entry!();
        TreeRefIter!();
        Error!();
        Find!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < 'a > TreeRefIter < 'a > { # [doc = " Instantiate an iterator from the given tree data."] pub fn from_bytes (data : & 'a [u8]) -> TreeRefIter < 'a > { TreeRefIter { data } } # [doc = " Follow a sequence of `path` components starting from this instance, and look them up in `odb` one by one using `buffer`"] # [doc = " until the last component is looked up and its tree entry is returned."] # [doc = ""] # [doc = " # Performance Notes"] # [doc = ""] # [doc = " Searching tree entries is currently done in sequence, which allows the search to be allocation free. It would be possible"] # [doc = " to reuse a vector and use a binary search instead, which might be able to improve performance over all."] # [doc = " However, a benchmark should be created first to have some data and see which trade-off to choose here."] pub fn lookup_entry < I , P > (& self , odb : impl crate :: Find , buffer : & 'a mut Vec < u8 > , path : I ,) -> Result < Option < tree :: Entry > , crate :: find :: Error > where I : IntoIterator < Item = P > , P : PartialEq < BStr > , { buffer . clear () ; let mut path = path . into_iter () . peekable () ; buffer . extend_from_slice (self . data) ; while let Some (component) = path . next () { match TreeRefIter :: from_bytes (buffer) . filter_map (Result :: ok) . find (| entry | component . eq (entry . filename)) { Some (entry) => { if path . peek () . is_none () { return Ok (Some (entry . into ())) ; } else { let next_id = entry . oid . to_owned () ; let obj = odb . try_find (& next_id , buffer) ? ; let Some (obj) = obj else { return Ok (None) } ; if ! obj . kind . is_tree () { return Ok (None) ; } } } None => return Ok (None) , } } Ok (None) } # [doc = " Like [`Self::lookup_entry()`], but takes any [`AsRef<Path>`](`std::path::Path`) directly via `relative_path`,"] # [doc = " a path relative to this tree."] # [doc = " `odb` and `buffer` are used to lookup intermediate trees."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " If any path component contains illformed UTF-8 and thus can't be converted to bytes on platforms which can't do so natively,"] # [doc = " the returned component will be empty which makes the lookup fail."] pub fn lookup_entry_by_path (& self , odb : impl crate :: Find , buffer : & 'a mut Vec < u8 > , relative_path : impl AsRef < std :: path :: Path > ,) -> Result < Option < tree :: Entry > , crate :: find :: Error > { use crate :: bstr :: ByteSlice ; self . lookup_entry (odb , buffer , relative_path . as_ref () . components () . map (| c : std :: path :: Component < '_ > | { gix_path :: os_str_into_bstr (c . as_os_str ()) . unwrap_or_else (| _ | "" . into ()) . as_bytes () }) ,) } }
    };
}

impl_115!();