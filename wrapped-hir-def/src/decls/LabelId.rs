macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! LabelId {
    () => {
        deps!();
        pub type LabelId = Idx < Label > ;
    };
}

LabelId!()