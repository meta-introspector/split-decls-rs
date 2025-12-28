macro_rules! deps {
    () => {
        ReferenceStyle!();
        ReferenceStage!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Default for ReferenceStage { fn default () -> Self { Self { name : String :: new () , style : ReferenceStyle :: Full , path : String :: new () , } } }
    };
}

impl_56!();