macro_rules! serde {
    () => {
        # [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] pub mod serde ;
    };
}

serde!()