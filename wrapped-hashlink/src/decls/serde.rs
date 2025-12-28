macro_rules! serde {
    () => {
        # [cfg (feature = "serde_impl")] pub mod serde ;
    };
}

serde!()