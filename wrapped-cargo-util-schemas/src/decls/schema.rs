macro_rules! schema {
    () => {
        # [cfg (feature = "unstable-schema")] pub mod schema ;
    };
}

schema!()