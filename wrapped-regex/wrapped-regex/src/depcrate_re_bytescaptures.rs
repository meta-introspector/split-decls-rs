// Generated macro for Captures (struct)
macro_rules! Depcrate_re_bytesCaptures {
() => {
// Module: crate::re_bytes
// Provides: {"Captures"}
// Dependencies: {}
# [doc = " Captures represents a group of captured byte strings for a single match."] # [doc = ""] # [doc = " The 0th capture always corresponds to the entire match. Each subsequent"] # [doc = " index corresponds to the next capture group in the regex. If a capture"] # [doc = " group is named, then the matched byte string is *also* available via the"] # [doc = " `name` method. (Note that the 0th capture is always unnamed and so must be"] # [doc = " accessed with the `at` method.)"] # [doc = ""] # [doc = " Positions returned from a capture group are always byte indices."] # [doc = ""] # [doc = " `'t` is the lifetime of the matched text."] pub struct Captures < 't > { text : & 't [u8] , slots : Vec < Option < usize > > , named_groups : Arc < HashMap < String , usize > > , }
};
}
