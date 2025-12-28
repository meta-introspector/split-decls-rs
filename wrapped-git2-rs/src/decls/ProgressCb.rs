macro_rules! deps {
    () => {
        PackBuilderStage!();
    };
}

macro_rules! ProgressCb {
    () => {
        deps!();
        pub type ProgressCb < 'a > = dyn FnMut (PackBuilderStage , u32 , u32) -> bool + 'a ;
    };
}

ProgressCb!();