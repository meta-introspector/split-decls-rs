// Generated macro for LookSet (struct)
macro_rules! Depcrate_util_lookLookSet {
() => {
// Module: crate::util::look
// Provides: {"LookSet"}
// Dependencies: {}
# [doc = " LookSet is a memory-efficient set of look-around assertions."] # [doc = ""] # [doc = " This is useful for efficiently tracking look-around assertions. For"] # [doc = " example, a [`thompson::NFA`](crate::nfa::thompson::NFA) provides properties"] # [doc = " that return `LookSet`s."] # [derive (Clone , Copy , Default , Eq , PartialEq)] pub struct LookSet { # [doc = " The underlying representation this set is exposed to make it possible"] # [doc = " to store it somewhere efficiently. The representation is that"] # [doc = " of a bitset, where each assertion occupies bit `i` where"] # [doc = " `i = Look::as_repr()`."] # [doc = ""] # [doc = " Note that users of this internal representation must permit the full"] # [doc = " range of `u16` values to be represented. For example, even if the"] # [doc = " current implementation only makes use of the 10 least significant bits,"] # [doc = " it may use more bits in a future semver compatible release."] pub bits : u32 , }
};
}
