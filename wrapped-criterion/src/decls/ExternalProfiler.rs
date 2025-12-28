macro_rules! ExternalProfiler {
    () => {
        # [doc = " Dummy profiler implementation, representing cases where the profiler is"] # [doc = " an external process (eg. perftools, etc.) which do not require start/stop"] # [doc = " hooks. This implementation does nothing and is used as the default."] pub struct ExternalProfiler ;
    };
}

ExternalProfiler!();