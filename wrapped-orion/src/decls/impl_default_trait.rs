macro_rules! impl_default_trait {
    () => {
        # [cfg (feature = "safe_api")] # [doc = " Macro that implements the `Default` trait using a CSPRNG."] macro_rules ! impl_default_trait (($ name : ident , $ size : expr) => (impl Default for $ name { # [cfg (feature = "safe_api")] # [cfg_attr (docsrs , doc (cfg (feature = "safe_api")))] # [doc = " Randomly generate using a CSPRNG with recommended size. Not available in `no_std` context."] fn default () -> $ name { let mut value = vec ! [0u8 ; $ size] ; crate :: util :: secure_rand_bytes (& mut value) . unwrap () ; $ name { value , original_length : $ size } } })) ;
    };
}

impl_default_trait!();