macro_rules! deps {
    () => {
        SerializationSink!();
        StringTableBuilder!();
        Counter!();
    };
}

macro_rules! Profiler {
    () => {
        deps!();
        pub struct Profiler { event_sink : Arc < SerializationSink > , string_table : StringTableBuilder , counter : Counter , }
    };
}

Profiler!()