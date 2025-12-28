macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! StopWatchSpan {
    () => {
        deps!();
        pub struct StopWatchSpan { pub time : Duration , pub instructions : Option < u64 > , pub memory : MemoryUsage , }
    };
}

StopWatchSpan!()