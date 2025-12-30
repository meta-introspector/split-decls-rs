// Generated macro for impl_355 (impl)
macro_rules! Depcrate_common_content_rangeimpl_355 {
() => {
// Module: crate::common::content_range
// Provides: {"impl_355"}
// Dependencies: {}
impl Header for ContentRange { fn name () -> & 'static HeaderName { & :: http :: header :: CONTENT_RANGE } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . and_then (| v | v . to_str () . ok ()) . and_then (| s | split_in_two (s , ' ')) . and_then (| (unit , spec) | { if unit != "bytes" { return None ; } let (range , complete_length) = split_in_two (spec , '/') ? ; let complete_length = if complete_length == "*" { None } else { Some (complete_length . parse () . ok () ?) } ; let range = if range == "*" { None } else { let (first_byte , last_byte) = split_in_two (range , '-') ? ; let first_byte = first_byte . parse () . ok () ? ; let last_byte = last_byte . parse () . ok () ? ; if last_byte < first_byte { return None ; } Some ((first_byte , last_byte)) } ; Some (ContentRange { range , complete_length , }) }) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { struct Adapter < 'a > (& 'a ContentRange) ; impl fmt :: Display for Adapter < '_ > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str ("bytes ") ? ; if let Some ((first_byte , last_byte)) = self . 0 . range { write ! (f , "{}-{}" , first_byte , last_byte) ? ; } else { f . write_str ("*") ? ; } f . write_str ("/") ? ; if let Some (v) = self . 0 . complete_length { write ! (f , "{}" , v) } else { f . write_str ("*") } } } values . extend (:: std :: iter :: once (util :: fmt (Adapter (self)))) ; } }
};
}
