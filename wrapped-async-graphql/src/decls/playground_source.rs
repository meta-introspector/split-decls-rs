macro_rules! playground_source {
    () => {
        # [cfg (feature = "playground")] mod playground_source ;
    };
}

playground_source!()