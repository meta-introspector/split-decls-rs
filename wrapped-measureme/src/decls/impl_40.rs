macro_rules! deps {
    () => {
        TimingGuard!();
        RawEvent!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'a > Drop for TimingGuard < 'a > { # [inline] fn drop (& mut self) { let raw_event = RawEvent :: new_interval (self . event_kind , self . event_id , self . thread_id , self . start_count , self . profiler . counter . since_start () ,) ; self . profiler . record_raw_event (& raw_event) ; } }
    };
}

impl_40!();