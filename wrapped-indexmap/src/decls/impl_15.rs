macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " <div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated: use borsh's <code>indexmap</code> feature instead.</span></div>"] impl < K , V , S > BorshDeserialize for IndexMap < K , V , S > where K : BorshDeserialize + Eq + Hash , V : BorshDeserialize , S : BuildHasher + Default , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { check_zst :: < K > () ? ; let vec = < Vec < (K , V) > > :: deserialize_reader (reader) ? ; Ok (vec . into_iter () . collect :: < IndexMap < K , V , S > > ()) } }
    };
}

impl_15!()