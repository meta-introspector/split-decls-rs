macro_rules! deps {
    () => {
        Number!();
    };
}

macro_rules! NumberFromString {
    () => {
        deps!();
        # [cfg (feature = "arbitrary_precision")] pub struct NumberFromString { pub value : Number , }
    };
}

NumberFromString!();