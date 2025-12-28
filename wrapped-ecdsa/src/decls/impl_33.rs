macro_rules! deps {
    () => {
        EcdsaCurve!();
        Signature!();
        MaxSize!();
        MaxOverhead!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < C > Serialize for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { serdect :: slice :: serialize_hex_upper_or_bin (& self . as_bytes () , serializer) } }
    };
}

impl_33!();