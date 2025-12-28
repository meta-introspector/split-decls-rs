macro_rules! deps {
    () => {
        ReferenceStyle!();
    };
}

macro_rules! ReferenceStage {
    () => {
        deps!();
        pub struct ReferenceStage { name : String , style : ReferenceStyle , path : String , }
    };
}

ReferenceStage!();