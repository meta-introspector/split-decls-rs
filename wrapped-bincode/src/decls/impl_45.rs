macro_rules! deps {
    () => {
        Encoder!();
        EncodeError!();
        Encode!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < K , V > Encode for BTreeMap < K , V > where K : Encode + Ord , V : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for (key , val) in self . iter () { key . encode (encoder) ? ; val . encode (encoder) ? ; } Ok (()) } }
    };
}

impl_45!();