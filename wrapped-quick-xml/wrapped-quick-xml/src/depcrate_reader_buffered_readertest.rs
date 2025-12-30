// Generated macro for test (module)
macro_rules! Depcrate_reader_buffered_readertest {
() => {
// Module: crate::reader::buffered_reader
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: reader :: test :: check ; use crate :: reader :: XmlSource ; # [doc = " Default buffer constructor just pass the byte array from the test"] fn identity < T > (input : T) -> T { input } check ! (# [test] read_event_impl , read_until_close , identity , & mut Vec :: new ()) ; }
};
}
