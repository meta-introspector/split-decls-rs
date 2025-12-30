// Generated macro for Dedup (struct)
macro_rules! Depcrate_connection_spacesDedup {
() => {
// Module: crate::connection::spaces
// Provides: {"Dedup"}
// Dependencies: {}
# [doc = " RFC4303-style sliding window packet number deduplicator."] # [doc = ""] # [doc = " A contiguous bitfield, where each bit corresponds to a packet number and the rightmost bit is"] # [doc = " always set. A set bit represents a packet that has been successfully authenticated. Bits left of"] # [doc = " the window are assumed to be set."] # [doc = ""] # [doc = " ```text"] # [doc = " ...xxxxxxxxx 1 0"] # [doc = "     ^        ^ ^"] # [doc = " window highest next"] # [doc = " ```"] pub (super) struct Dedup { window : Window , # [doc = " Lowest packet number higher than all yet authenticated."] next : u64 , }
};
}
