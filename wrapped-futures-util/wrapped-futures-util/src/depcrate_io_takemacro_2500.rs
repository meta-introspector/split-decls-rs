// Generated macro for macro_2500 (macro)
macro_rules! Depcrate_io_takemacro_2500 {
() => {
// Module: crate::io::take
// Provides: {"macro_2500"}
// Dependencies: {}
pin_project ! { # [doc = " Reader for the [`take`](super::AsyncReadExt::take) method."] # [derive (Debug)] # [must_use = "readers do nothing unless you `.await` or poll them"] pub struct Take < R > { # [pin] inner : R , limit : u64 , } }
};
}
