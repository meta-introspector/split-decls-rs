macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < K , V , S > Encode for HashMap < K , V , S > where K : Encode , V : Encode , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for (k , v) in self . iter () { Encode :: encode (k , encoder) ? ; Encode :: encode (v , encoder) ? ; } Ok (()) } }
    };
}

impl_130!();