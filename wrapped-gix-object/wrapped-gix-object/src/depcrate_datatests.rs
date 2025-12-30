// Generated macro for tests (module)
macro_rules! Depcrate_datatests {
() => {
// Module: crate::data
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn size_of_object () { # [cfg (target_pointer_width = "64")] assert_eq ! (std :: mem :: size_of ::< Data <'_ >> () , 24 , "this shouldn't change unnoticed") ; # [cfg (target_pointer_width = "32")] assert_eq ! (std :: mem :: size_of ::< Data <'_ >> () , 12 , "this shouldn't change unnoticed") ; } }
};
}
