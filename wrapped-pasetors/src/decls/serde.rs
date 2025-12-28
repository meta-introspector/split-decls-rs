macro_rules! serde {
    () => {
        # [cfg (feature = "serde")] # [doc = " Serialization and deserialization support for various types."] mod serde ;
    };
}

serde!()