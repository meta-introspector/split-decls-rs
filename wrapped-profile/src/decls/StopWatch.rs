macro_rules! deps {
    () => {
        MemoryUsage!();
    };
}

macro_rules! StopWatch {
    () => {
        deps!();
        pub struct StopWatch { time : Instant , # [cfg (all (target_os = "linux" , not (target_env = "ohos")))] counter : Option < perf_event :: Counter > , memory : MemoryUsage , }
    };
}

StopWatch!()