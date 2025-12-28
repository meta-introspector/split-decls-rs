macro_rules! future {
    () => {
        # [cfg (feature = "std")] pub mod future ;
    };
}

future!()