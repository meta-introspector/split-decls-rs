// Generated macro for Replacer (trait)
macro_rules! Depcrate_re_bytesReplacer {
() => {
// Module: crate::re_bytes
// Provides: {"Replacer"}
// Dependencies: {}
# [doc = " Replacer describes types that can be used to replace matches in a byte"] # [doc = " string."] # [doc = ""] # [doc = " In general, users of this crate shouldn't need to implement this trait,"] # [doc = " since implementations are already provided for `&[u8]` and"] # [doc = " `FnMut(&Captures) -> Vec<u8>`, which covers most use cases."] pub trait Replacer { # [doc = " Appends text to `dst` to replace the current match."] # [doc = ""] # [doc = " The current match is represented by `caps`, which is guaranteed to"] # [doc = " have a match at capture group `0`."] # [doc = ""] # [doc = " For example, a no-op replacement would be"] # [doc = " `dst.extend(caps.at(0).unwrap())`."] fn replace_append (& mut self , caps : & Captures , dst : & mut Vec < u8 >) ; # [doc = " Return a fixed unchanging replacement byte string."] # [doc = ""] # [doc = " When doing replacements, if access to `Captures` is not needed (e.g.,"] # [doc = " the replacement byte string does not need `$` expansion), then it can"] # [doc = " be beneficial to avoid finding sub-captures."] # [doc = ""] # [doc = " In general, this is called once for every call to `replacen`."] fn no_expansion < 'r > (& 'r mut self) -> Option < Cow < 'r , [u8] > > { None } }
};
}
