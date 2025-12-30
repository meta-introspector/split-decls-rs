// Generated macro for use_75 (use)
macro_rules! Depcrate_serializationuse_75 {
() => {
// Module: crate::serialization
// Provides: {"use_75"}
// Dependencies: {}
# [doc = " This module implements the \"container\" file format that `measureme` uses for"] # [doc = " storing things on disk. The format supports storing three independent"] # [doc = " streams of data: one for events, one for string data, and one for string"] # [doc = " index data (in theory it could support an arbitrary number of separate"] # [doc = " streams but three is all we need). The data of each stream is split into"] # [doc = " \"pages\", where each page has a small header designating what kind of"] # [doc = " data it is (i.e. event, string data, or string index), and the length of"] # [doc = " the page."] # [doc = ""] # [doc = " Pages of different kinds can be arbitrarily interleaved. The headers allow"] # [doc = " for reconstructing each of the streams later on. An example file might thus"] # [doc = " look like this:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " | file header | page (events) | page (string data) | page (events) | page (string index) |"] # [doc = " ```"] # [doc = ""] # [doc = " The exact encoding of a page is:"] # [doc = ""] # [doc = " | byte slice              | contents                                |"] # [doc = " |-------------------------|-----------------------------------------|"] # [doc = " | &[0 .. 1]               | page tag                                |"] # [doc = " | &[1 .. 5]               | page size as little endian u32          |"] # [doc = " | &[5 .. (5 + page_size)] | page contents (exactly page_size bytes) |"] # [doc = ""] # [doc = " A page is immediately followed by the next page, without any padding."] use parking_lot :: Mutex ;
};
}
