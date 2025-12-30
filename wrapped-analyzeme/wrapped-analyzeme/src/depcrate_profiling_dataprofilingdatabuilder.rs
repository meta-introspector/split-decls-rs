// Generated macro for ProfilingDataBuilder (struct)
macro_rules! Depcrate_profiling_dataProfilingDataBuilder {
() => {
// Module: crate::profiling_data
// Provides: {"ProfilingDataBuilder"}
// Dependencies: {}
# [doc = " A `ProfilingDataBuilder` allows for programmatically building"] # [doc = " `ProfilingData` objects. This is useful for writing tests that expect"] # [doc = " `ProfilingData` with predictable events (and especially timestamps) in it."] # [doc = ""] # [doc = " `ProfilingDataBuilder` provides a convenient interface but its"] # [doc = " implementation might not be efficient, which why it should only be used for"] # [doc = " writing tests and other things that are not performance sensitive."] pub struct ProfilingDataBuilder { event_sink : SerializationSink , string_table_data_sink : Arc < SerializationSink > , string_table_index_sink : Arc < SerializationSink > , string_table : StringTableBuilder , }
};
}
