macro_rules! deps {
    () => {
        PasswordHash!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] # [doc = " `PasswordHash` deserializes from a [`String`](std::string::String)."] impl < 'de > Deserialize < 'de > for PasswordHash { fn deserialize < D > (deserializer : D) -> Result < PasswordHash , D :: Error > where D : Deserializer < 'de > , { let encoded_str = String :: deserialize (deserializer) ? ; PasswordHash :: from_encoded (& encoded_str) . map_err (de :: Error :: custom) } }
    };
}

impl_640!();