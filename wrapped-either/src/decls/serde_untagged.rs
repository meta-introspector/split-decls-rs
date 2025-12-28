macro_rules! serde_untagged {
    () => {
        # [cfg (feature = "serde")] pub mod serde_untagged ;
    };
}

serde_untagged!()