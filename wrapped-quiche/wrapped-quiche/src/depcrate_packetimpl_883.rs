// Generated macro for impl_883 (impl)
macro_rules! Depcrate_packetimpl_883 {
() => {
// Module: crate::packet
// Provides: {"impl_883"}
// Dependencies: {}
impl Epoch { # [doc = " Returns an ordered slice containing the `Epoch`s that fit in the"] # [doc = " provided `range`."] pub fn epochs (range : RangeInclusive < Epoch >) -> & 'static [Epoch] { & EPOCHS [* range . start () as usize ..= * range . end () as usize] } pub const fn count () -> usize { 3 } }
};
}
