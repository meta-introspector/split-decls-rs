macro_rules! deps {
    () => {
        Instructions!();
    };
}

macro_rules! HwCounterType {
    () => {
        deps!();
        enum HwCounterType { Instructions , Irqs , Raw0420 , }
    };
}

HwCounterType!();