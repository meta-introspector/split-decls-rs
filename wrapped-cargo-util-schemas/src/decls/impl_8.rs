macro_rules! deps {
    () => {
        PackageIdSpec!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [cfg (feature = "unstable-schema")] impl schemars :: JsonSchema for PackageIdSpec { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "PackageIdSpec" . into () } fn json_schema (generator : & mut schemars :: SchemaGenerator) -> schemars :: Schema { < String as schemars :: JsonSchema > :: json_schema (generator) } }
    };
}

impl_8!();