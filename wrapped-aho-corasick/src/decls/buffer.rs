macro_rules! buffer {
    () => {
        # [cfg (feature = "std")] pub (crate) mod buffer ;
    };
}

buffer!()