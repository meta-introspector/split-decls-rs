// Generated macro for tests (module)
macro_rules! Depcrate_stream_binary_writertests {
() => {
// Module: crate::stream::binary_writer
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: { fs :: File , io :: Cursor , path :: Path } ; use crate :: { stream :: BinaryReader , Value } ; fn test_roundtrip < P : AsRef < Path > > (path : P) { let reader = File :: open (path) . unwrap () ; let streaming_parser = BinaryReader :: new (reader) ; let value_to_encode = Value :: from_events (streaming_parser) . unwrap () ; let mut buf = Cursor :: new (Vec :: new ()) ; value_to_encode . to_writer_binary (& mut buf) . unwrap () ; let buf_inner = buf . into_inner () ; let streaming_parser = BinaryReader :: new (Cursor :: new (buf_inner)) ; let events : Vec < Result < _ , _ > > = streaming_parser . collect () ; let value_decoded_from_encode = Value :: from_events (events) . unwrap () ; assert_eq ! (value_to_encode , value_decoded_from_encode) ; } # [test] fn bplist_roundtrip () { test_roundtrip ("./tests/data/binary.plist") } # [test] fn utf16_roundtrip () { test_roundtrip ("./tests/data/utf16_bplist.plist") } # [test] fn nskeyedarchiver_roundtrip () { test_roundtrip ("./tests/data/binary_NSKeyedArchiver.plist") } }
};
}
