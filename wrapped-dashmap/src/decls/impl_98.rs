macro_rules! deps {
    () => {
        DashSet!();
        DashSetVisitor!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'de , K , S > Visitor < 'de > for DashSetVisitor < K , S > where K : Deserialize < 'de > + Eq + Hash , S : BuildHasher + Clone + Default , { type Value = DashSet < K , S > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a DashSet") } fn visit_seq < M > (self , mut access : M) -> Result < Self :: Value , M :: Error > where M : SeqAccess < 'de > , { let map = DashSet :: with_capacity_and_hasher (access . size_hint () . unwrap_or (0) , Default :: default ()) ; while let Some (key) = access . next_element () ? { map . insert (key) ; } Ok (map) } }
    };
}

impl_98!();