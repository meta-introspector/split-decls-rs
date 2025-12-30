// Generated macro for str_cow_to_bytes (function)
macro_rules! Depcrate_eventsstr_cow_to_bytes {
() => {
// Module: crate::events
// Provides: {"str_cow_to_bytes"}
// Dependencies: {}
# [inline] fn str_cow_to_bytes < 'a , C : Into < Cow < 'a , str > > > (content : C) -> Cow < 'a , [u8] > { match content . into () { Cow :: Borrowed (s) => Cow :: Borrowed (s . as_bytes ()) , Cow :: Owned (s) => Cow :: Owned (s . into_bytes ()) , } }
};
}
