macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " <div class=\"stab deprecated\"><span class=\"emoji\">👎</span><span>Deprecated: use borsh's <code>indexmap</code> feature instead.</span></div>"] impl < K , V , S > BorshSerialize for IndexMap < K , V , S > where K : BorshSerialize , V : BorshSerialize , { # [inline] fn serialize < W : Write > (& self , writer : & mut W) -> Result < () > { check_zst :: < K > () ? ; let iterator = self . iter () ; u32 :: try_from (iterator . len ()) . map_err (| _ | ErrorKind :: InvalidData) ? . serialize (writer) ? ; for (key , value) in iterator { key . serialize (writer) ? ; value . serialize (writer) ? ; } Ok (()) } }
    };
}

impl_14!()