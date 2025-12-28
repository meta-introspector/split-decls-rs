macro_rules! deps {
    () => {
        Signature!();
        SignatureSize!();
        SignatureBytes!();
        EcdsaCurve!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , C > Deserialize < 'de > for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { let mut bytes = SignatureBytes :: < C > :: default () ; serdect :: array :: deserialize_hex_or_bin (& mut bytes , deserializer) ? ; Self :: try_from (bytes . as_slice ()) . map_err (de :: Error :: custom) } }
    };
}

impl_159!();