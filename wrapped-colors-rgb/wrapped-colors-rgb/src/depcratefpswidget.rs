// Generated macro for FpsWidget (struct)
macro_rules! DepcrateFpsWidget {
() => {
// Module: crate
// Provides: {"FpsWidget"}
// Dependencies: {}
# [doc = " A widget that displays the current frames per second"] # [derive (Debug)] struct FpsWidget { # [doc = " The number of elapsed frames that have passed - used to calculate the fps"] frame_count : usize , # [doc = " The last instant that the fps was calculated"] last_instant : Instant , # [doc = " The current frames per second"] fps : Option < f32 > , }
};
}
