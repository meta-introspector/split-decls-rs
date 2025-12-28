macro_rules! StatsLevel {
    () => {
        # [doc = " StatsLevel can be used to reduce statistics overhead by skipping certain"] # [doc = " types of stats in the stats collection process."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (u8)] pub enum StatsLevel { # [doc = " Disable all metrics"] DisableAll = 0 , # [doc = " Disable timer stats, and skip histogram stats"] ExceptHistogramOrTimers = 2 , # [doc = " Skip timer stats"] ExceptTimers , # [doc = " Collect all stats except time inside mutex lock AND time spent on"] # [doc = " compression."] ExceptDetailedTimers , # [doc = " Collect all stats except the counters requiring to get time inside the"] # [doc = " mutex lock."] ExceptTimeForMutex , # [doc = " Collect all stats, including measuring duration of mutex operations."] # [doc = " If getting time is expensive on the platform to run, it can"] # [doc = " reduce scalability to more threads, especially for writes."] All , }
    };
}

StatsLevel!()