// Generated macro for Collect (struct)
macro_rules! Depcrate_status_iterCollect {
() => {
// Module: crate::status::iter
// Provides: {"Collect"}
// Dependencies: {}
struct Collect { # [cfg (feature = "parallel")] tx : std :: sync :: mpsc :: Sender < Item > , # [cfg (not (feature = "parallel"))] items : Vec < Item > , }
};
}
