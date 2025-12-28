macro_rules! deps {
    () => {
        AsymmetricSecretKey!();
        Error!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        # [cfg (all (feature = "serde" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "serde" , feature = "std"))))] impl < 'de , V > serde :: Deserialize < 'de > for AsymmetricSecretKey < V > where AsymmetricSecretKey < V > : TryFrom < & 'de str > , < AsymmetricSecretKey < V > as TryFrom < & 'de str > > :: Error : std :: fmt :: Display , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let paserk_string = < & str > :: deserialize (deserializer) ? ; TryFrom :: try_from (paserk_string) . map_err (serde :: de :: Error :: custom) } }
    };
}

impl_139!()