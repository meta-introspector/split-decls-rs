macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [doc = " <div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated: use borsh's <code>indexmap</code> feature instead.</span></div>"] impl < T , S > BorshDeserialize for IndexSet < T , S > where T : BorshDeserialize + Eq + Hash , S : BuildHasher + Default , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { check_zst :: < T > () ? ; let vec = < Vec < T > > :: deserialize_reader (reader) ? ; Ok (vec . into_iter () . collect :: < IndexSet < T , S > > ()) } }
    };
}

impl_17!()