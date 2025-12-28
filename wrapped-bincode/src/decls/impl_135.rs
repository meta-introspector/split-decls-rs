macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < T , S > Encode for HashSet < T , S > where T : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for item in self . iter () { item . encode (encoder) ? ; } Ok (()) } }
    };
}

impl_135!();