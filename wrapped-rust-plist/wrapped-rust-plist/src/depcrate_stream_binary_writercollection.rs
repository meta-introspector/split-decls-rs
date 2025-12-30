// Generated macro for Collection (struct)
macro_rules! Depcrate_stream_binary_writerCollection {
() => {
// Module: crate::stream::binary_writer
// Provides: {"Collection"}
// Dependencies: {}
struct Collection { ty : CollectionType , # [doc = " The number of elements in an array or (key, value) pairs in a dictionary."] # [doc = " Unclosed dictionaries have a `len` equal to the number of keys plus the number of values"] # [doc = " written so far. This is fixed up in `write_end_collection`."] len : usize , # [doc = " The number of events to skip to get to the next element after the collection."] skip : usize , object_ref : Option < ObjectRef > , }
};
}
