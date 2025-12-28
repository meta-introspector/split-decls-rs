macro_rules! deps {
    () => {
        RecordFormat!();
    };
}

macro_rules! FormatFn {
    () => {
        deps!();
        pub (crate) type FormatFn = Box < dyn RecordFormat + Sync + Send > ;
    };
}

FormatFn!()