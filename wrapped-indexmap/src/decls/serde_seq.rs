macro_rules! serde_seq {
    () => {
        # [cfg (feature = "serde")] # [cfg_attr (docsrs , doc (cfg (feature = "serde")))] pub mod serde_seq ;
    };
}

serde_seq!()