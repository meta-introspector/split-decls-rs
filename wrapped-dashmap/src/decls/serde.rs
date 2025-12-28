macro_rules! serde {
    () => {
        # [cfg (feature = "serde")] mod serde ;
    };
}

serde!();