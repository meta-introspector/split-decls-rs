macro_rules! deps {
    () => {
        EncodeError!();
        Encoder!();
        Encode!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T > Encode for BinaryHeap < T > where T : Encode + Ord , { fn encode < E : Encoder > (& self , encoder : & mut E) -> Result < () , EncodeError > { crate :: enc :: encode_slice_len (encoder , self . len ()) ? ; for val in self . iter () { val . encode (encoder) ? ; } Ok (()) } }
    };
}

impl_42!()