macro_rules! deps {
    () => {
        DashMapVisitor!();
        DashMap!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'de , K , V , S > Visitor < 'de > for DashMapVisitor < K , V , S > where K : Deserialize < 'de > + Eq + Hash , V : Deserialize < 'de > , S : BuildHasher + Clone + Default , { type Value = DashMap < K , V , S > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a DashMap") } fn visit_map < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : MapAccess < 'de > , { let map = DashMap :: with_capacity_and_hasher (access . size_hint () . unwrap_or (0) , Default :: default ()) ; while let Some ((key , value)) = access . next_entry () ? { map . insert (key , value) ; } Ok (map) } }
    };
}

impl_93!()