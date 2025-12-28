macro_rules! BinsConfig {
    () => {
        # [derive (Debug , Deserialize , Clone)] pub struct BinsConfig { # [serde (flatten)] pub paths : HashMap < String , PathBuf > , }
    };
}

BinsConfig!()