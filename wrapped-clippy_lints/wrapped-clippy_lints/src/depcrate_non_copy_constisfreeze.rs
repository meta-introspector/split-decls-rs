// Generated macro for IsFreeze (enum)
macro_rules! Depcrate_non_copy_constIsFreeze {
() => {
// Module: crate::non_copy_const
// Provides: {"IsFreeze"}
// Dependencies: {}
# [derive (Clone , Copy)] enum IsFreeze { # [doc = " The type and all possible values are `Freeze`"] Yes , # [doc = " The type itself is non-`Freeze`, but not all values are."] Maybe , # [doc = " The type and all possible values are non-`Freeze`"] No , }
};
}
