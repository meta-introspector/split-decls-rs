macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! DelayLoadDescriptorIterator {
    () => {
        deps!();
        # [doc = " A fallible iterator for the descriptors in the delay-load data directory."] # [derive (Debug , Clone)] pub struct DelayLoadDescriptorIterator < 'data > { data : Bytes < 'data > , null : bool , }
    };
}

DelayLoadDescriptorIterator!();