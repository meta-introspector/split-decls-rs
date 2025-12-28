macro_rules! deps {
    () => {
        StoreBulkMut!();
        LiteMapVisitor!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < 'de , K , V , R > Visitor < 'de > for LiteMapVisitor < K , V , R > where K : Deserialize < 'de > + Ord , V : Deserialize < 'de > , R : StoreBulkMut < K , V > , { type Value = LiteMap < K , V , R > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a map produced by LiteMap") } fn visit_seq < S > (self , mut access : S) -> Result < Self :: Value , S :: Error > where S : SeqAccess < 'de > , { let mut map = LiteMap :: with_capacity (access . size_hint () . unwrap_or (0)) ; let mut out_of_order = Vec :: new () ; while let Some ((key , value)) = access . next_element () ? { if let Some ((key , value)) = map . try_append (key , value) { out_of_order . push ((key , value)) ; } } if ! out_of_order . is_empty () { map . extend (out_of_order) ; } Ok (map) } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let mut map = LiteMap :: with_capacity (access . size_hint () . unwrap_or (0)) ; let mut out_of_order = Vec :: new () ; while let Some ((key , value)) = access . next_entry () ? { if let Some ((key , value)) = map . try_append (key , value) { out_of_order . push ((key , value)) ; } } if ! out_of_order . is_empty () { map . extend (out_of_order) ; } Ok (map) } }
    };
}

impl_60!()