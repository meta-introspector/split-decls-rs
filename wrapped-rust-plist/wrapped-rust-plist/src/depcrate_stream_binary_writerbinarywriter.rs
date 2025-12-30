// Generated macro for BinaryWriter (struct)
macro_rules! Depcrate_stream_binary_writerBinaryWriter {
() => {
// Module: crate::stream::binary_writer
// Provides: {"BinaryWriter"}
// Dependencies: {}
pub struct BinaryWriter < W : Write > { writer : PosWriter < W > , events : Vec < Event > , dictionary_key_events : Vec < usize > , values : IndexMap < Value < 'static > , ValueState > , # [doc = " Pointers into `events` for each of the currently unclosed `Collection` events."] collection_stack : Vec < usize > , # [doc = " The number of `Collection` and unique `Value` events in `events`."] num_objects : usize , }
};
}
