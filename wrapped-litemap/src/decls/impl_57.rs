macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < K , V , R > Serialize for LiteMap < K , V , R > where K : Serialize , V : Serialize , R : Store < K , V > , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { let k_is_num_or_string = self . values . lm_get (0) . is_some_and (| (k , _) | super :: serde_helpers :: is_num_or_string (k)) ; if ! k_is_num_or_string { let mut seq = serializer . serialize_seq (Some (self . len ())) ? ; for index in 0 .. self . len () { # [expect (clippy :: unwrap_used)] seq . serialize_element (& self . get_indexed (index) . unwrap ()) ? ; } return seq . end () ; } } let mut map = serializer . serialize_map (Some (self . len ())) ? ; for index in 0 .. self . len () { # [expect (clippy :: unwrap_used)] let (k , v) = self . get_indexed (index) . unwrap () ; map . serialize_entry (k , v) ? ; } map . end () } }
    };
}

impl_57!()