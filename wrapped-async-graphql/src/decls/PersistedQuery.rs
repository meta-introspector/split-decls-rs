macro_rules! PersistedQuery {
    () => {
        # [derive (Deserialize)] struct PersistedQuery { version : i32 , # [serde (rename = "sha256Hash")] sha256_hash : String , }
    };
}

PersistedQuery!()