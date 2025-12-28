macro_rules! deps {
    () => {
        Profiler!();
        StringId!();
        EventId!();
    };
}

macro_rules! TimingGuard {
    () => {
        deps!();
        # [doc = " When dropped, this `TimingGuard` will record an \"end\" event in the"] # [doc = " `Profiler` it was created by."] # [must_use] pub struct TimingGuard < 'a > { profiler : & 'a Profiler , event_id : EventId , event_kind : StringId , thread_id : u32 , start_count : u64 , }
    };
}

TimingGuard!();