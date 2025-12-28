macro_rules! deps {
    () => {
        Pairs!();
        Pair!();
        RuleType!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'i , R : RuleType > Pairs < 'i , R > { # [doc = " Create a new `Pairs` iterator containing just the single `Pair`."] pub fn single (pair : Pair < 'i , R >) -> Self { let end = pair . pair () ; pairs :: new (pair . queue , pair . input , Some (pair . line_index) , pair . start , end ,) } }
    };
}

impl_39!()