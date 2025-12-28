macro_rules! serde_untagged_optional {
    () => {
        # [cfg (feature = "serde")] pub mod serde_untagged_optional ;
    };
}

serde_untagged_optional!();