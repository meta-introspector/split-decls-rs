macro_rules! deps {
    () => {
        Error!();
        Id!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        # [cfg (all (feature = "paserk" , feature = "serde" , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "paserk" , feature = "serde" , feature = "std"))))] impl < 'de > serde :: Deserialize < 'de > for Id { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let paserk_id = < & str > :: deserialize (deserializer) ? ; TryFrom :: try_from (paserk_id) . map_err (serde :: de :: Error :: custom) } }
    };
}

impl_143!();