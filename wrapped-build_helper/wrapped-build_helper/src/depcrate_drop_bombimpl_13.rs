// Generated macro for impl_13 (impl)
macro_rules! Depcrate_drop_bombimpl_13 {
() => {
// Module: crate::drop_bomb
// Provides: {"impl_13"}
// Dependencies: {}
impl DropBomb { # [doc = " Arm a [`DropBomb`]. If the value is dropped without being [`defused`][Self::defused], then"] # [doc = " it will panic. It is expected that the command wrapper uses `#[track_caller]` to help"] # [doc = " propagate the caller location."] # [track_caller] pub fn arm < S : AsRef < OsStr > > (command : S) -> DropBomb { DropBomb { command : command . as_ref () . into () , defused : false , armed_location : * panic :: Location :: caller () , } } pub fn get_created_location (& self) -> panic :: Location < 'static > { self . armed_location } # [doc = " Defuse the [`DropBomb`]. This will prevent the drop bomb from panicking when dropped."] pub fn defuse (& mut self) { self . defused = true ; } }
};
}
