macro_rules! deps {
    () => {
        TomlValueWrapper!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl JsonSchema for TomlValueWrapper { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "TomlValue" . into () } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { generator . subschema_for :: < serde_json :: Value > () . into () } }
    };
}

impl_198!();