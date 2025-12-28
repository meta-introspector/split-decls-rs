macro_rules! deps {
    () => {
        Encode!();
        EncodeError!();
        Encoder!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T > Encode for BTreeSet < T > where T : Encode + Ord , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for item in self . iter () { item . encode (encoder) ? ; } Ok (()) } }
    };
}

impl_48!();