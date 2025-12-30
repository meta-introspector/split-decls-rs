// Generated macro for Collect (struct)
macro_rules! Depcrate_dirwalk_iterCollect {
() => {
// Module: crate::dirwalk::iter
// Provides: {"Collect"}
// Dependencies: {}
struct Collect { # [cfg (feature = "parallel")] tx : std :: sync :: mpsc :: Sender < Item > , # [cfg (not (feature = "parallel"))] items : Vec < Item > , }
};
}
