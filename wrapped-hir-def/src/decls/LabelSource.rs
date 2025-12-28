macro_rules! deps {
    () => {
        LabelPtr!();
    };
}

macro_rules! LabelSource {
    () => {
        deps!();
        pub type LabelSource = InFile < LabelPtr > ;
    };
}

LabelSource!();