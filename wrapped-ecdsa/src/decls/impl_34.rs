macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        MaxOverhead!();
        SignatureBytes!();
        MaxSize!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , C > Deserialize < 'de > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let mut buf = SignatureBytes :: < C > :: default () ; let slice = serdect :: slice :: deserialize_hex_or_bin (& mut buf , deserializer) ? ; Self :: try_from (slice) . map_err (de :: Error :: custom) } }
    };
}

impl_34!()