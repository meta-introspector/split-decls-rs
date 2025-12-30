// Generated macro for destroy (function)
macro_rules! Depcrate_destroydestroy {
() => {
// Module: crate::destroy
// Provides: {"destroy"}
// Dependencies: {}
# [doc = " Destroy mode activated by pressing `d`"] pub fn destroy (frame : & mut Frame < '_ >) { let frame_count = frame . count () . saturating_sub (DELAY) ; if frame_count == 0 { return ; } let area = frame . area () ; let buf = frame . buffer_mut () ; drip (frame_count , area , buf) ; text (frame_count , area , buf) ; }
};
}
