macro_rules! deps {
    () => {
        Error!();
        SymmetricKey!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        # [cfg (all (feature = "serde" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "serde" , feature = "std"))))] impl < 'de , V > serde :: Deserialize < 'de > for SymmetricKey < V > where SymmetricKey < V > : TryFrom < & 'de str > , < SymmetricKey < V > as TryFrom < & 'de str > > :: Error : std :: fmt :: Display , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let paserk_string = < & str > :: deserialize (deserializer) ? ; TryFrom :: try_from (paserk_string) . map_err (serde :: de :: Error :: custom) } }
    };
}

impl_141!()