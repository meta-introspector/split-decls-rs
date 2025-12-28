macro_rules! task {
    () => {
        # [cfg (feature = "std")] pub mod task ;
    };
}

task!();