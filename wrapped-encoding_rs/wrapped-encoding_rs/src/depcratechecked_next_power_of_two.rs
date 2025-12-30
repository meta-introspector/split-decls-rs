// Generated macro for checked_next_power_of_two (function)
macro_rules! Depcratechecked_next_power_of_two {
() => {
// Module: crate
// Provides: {"checked_next_power_of_two"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [inline (always)] fn checked_next_power_of_two (opt : Option < usize >) -> Option < usize > { opt . map (| n | n . next_power_of_two ()) }
};
}
