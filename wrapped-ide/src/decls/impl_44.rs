macro_rules! deps {
    () => {
        RangeInfo!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T > RangeInfo < T > { pub fn new (range : TextRange , info : T) -> RangeInfo < T > { RangeInfo { range , info } } }
    };
}

impl_44!()