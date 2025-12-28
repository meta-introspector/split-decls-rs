macro_rules! deps {
    () => {
        RunnableKind!();
    };
}

macro_rules! should_skip_runnable {
    () => {
        deps!();
        fn should_skip_runnable (kind : & RunnableKind , binary_target : bool) -> bool { match kind { RunnableKind :: Bin => ! binary_target , _ => false , } }
    };
}

should_skip_runnable!()