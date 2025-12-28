macro_rules! deps {
    () => {
        Execution!();
    };
}

macro_rules! fence_acqrel {
    () => {
        deps!();
        fn fence_acqrel (execution : & mut Execution) { fence_acq (execution) ; fence_rel (execution) ; }
    };
}

fence_acqrel!();