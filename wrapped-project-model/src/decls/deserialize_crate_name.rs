macro_rules! deserialize_crate_name {
    () => {
        fn deserialize_crate_name < 'de , D > (de : D) -> std :: result :: Result < CrateName , D :: Error > where D : de :: Deserializer < 'de > , { let name = String :: deserialize (de) ? ; CrateName :: new (& name) . map_err (| err | de :: Error :: custom (format ! ("invalid crate name: {err:?}"))) }
    };
}

deserialize_crate_name!();