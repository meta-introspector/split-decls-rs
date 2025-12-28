macro_rules! deps {
    () => {
        DataFormat!();
    };
}

macro_rules! FullReset {
    () => {
        deps!();
        # [doc = " Full reset of the state, including zeroing memory."] # [doc = ""] # [doc = " Requires to provide new data format."] pub struct FullReset (pub DataFormat) ;
    };
}

FullReset!();