macro_rules! serde {
    () => {
        # [cfg (feature = "serde")] pub mod serde ;
    };
}

serde!()