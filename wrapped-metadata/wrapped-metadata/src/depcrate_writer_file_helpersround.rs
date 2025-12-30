// Generated macro for round (function)
macro_rules! Depcrate_writer_file_helpersround {
() => {
// Module: crate::writer::file::helpers
// Provides: {"round"}
// Dependencies: {}
pub fn round (size : usize , round : usize) -> usize { let round = round - 1 ; (size + round) & ! round }
};
}
