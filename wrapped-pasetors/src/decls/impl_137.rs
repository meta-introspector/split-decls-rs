macro_rules! deps {
    () => {
        Error!();
        AsymmetricPublicKey!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        # [cfg (all (feature = "serde" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "serde" , feature = "std"))))] impl < 'de , V > serde :: Deserialize < 'de > for AsymmetricPublicKey < V > where AsymmetricPublicKey < V > : TryFrom < & 'de str > , < AsymmetricPublicKey < V > as TryFrom < & 'de str > > :: Error : std :: fmt :: Display , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let paserk_string = < & str > :: deserialize (deserializer) ? ; TryFrom :: try_from (paserk_string) . map_err (serde :: de :: Error :: custom) } }
    };
}

impl_137!()