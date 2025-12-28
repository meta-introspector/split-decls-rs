macro_rules! deps {
    () => {
        IndexSet!();
        IndexSetVisitor!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'de , T , S > Visitor < 'de > for IndexSetVisitor < T , S > where T : Deserialize < 'de > + Eq + Hash , S : Default + BuildHasher , { type Value = IndexSet < T , S > ; fn expecting (& self , formatter : & mut Formatter < '_ >) -> fmt :: Result { write ! (formatter , "a set") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let capacity = cautious_capacity :: < T , () > (seq . size_hint ()) ; let mut values = IndexSet :: with_capacity_and_hasher (capacity , S :: default ()) ; while let Some (value) = seq . next_element () ? { values . insert (value) ; } Ok (values) } }
    };
}

impl_29!();