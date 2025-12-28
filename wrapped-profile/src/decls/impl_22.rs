macro_rules! deps {
    () => {
        StopWatchSpan!();
        MemoryUsage!();
        StopWatch!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl StopWatch { pub fn start () -> StopWatch { # [cfg (all (target_os = "linux" , not (target_env = "ohos")))] let counter = { use std :: sync :: OnceLock ; static PERF_ENABLED : OnceLock < bool > = OnceLock :: new () ; if * PERF_ENABLED . get_or_init (| | std :: env :: var_os ("RA_DISABLE_PERF") . is_none ()) { let mut counter = perf_event :: Builder :: new () . build () . map_err (| err | eprintln ! ("Failed to create perf counter: {err}")) . ok () ; if let Some (counter) = & mut counter && let Err (err) = counter . enable () { eprintln ! ("Failed to start perf counter: {err}") } counter } else { None } } ; let memory = MemoryUsage :: now () ; let time = Instant :: now () ; StopWatch { time , # [cfg (all (target_os = "linux" , not (target_env = "ohos")))] counter , memory , } } pub fn elapsed (& mut self) -> StopWatchSpan { let time = self . time . elapsed () ; # [cfg (all (target_os = "linux" , not (target_env = "ohos")))] let instructions = self . counter . as_mut () . and_then (| it | { it . read () . map_err (| err | eprintln ! ("Failed to read perf counter: {err}")) . ok () }) ; # [cfg (all (target_os = "linux" , target_env = "ohos"))] let instructions = None ; # [cfg (not (target_os = "linux"))] let instructions = None ; let memory = MemoryUsage :: now () - self . memory ; StopWatchSpan { time , instructions , memory } } }
    };
}

impl_22!()