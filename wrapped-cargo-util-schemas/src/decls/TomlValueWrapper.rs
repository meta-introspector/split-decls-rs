macro_rules! TomlValueWrapper {
    () => {
        # [derive (Serialize , Deserialize)] pub struct TomlValueWrapper (pub TomlValue) ;
    };
}

TomlValueWrapper!();