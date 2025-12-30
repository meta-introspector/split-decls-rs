// Generated macro for GuardExt (trait)
macro_rules! Depcrate_guardGuardExt {
() => {
// Module: crate::guard
// Provides: {"GuardExt"}
// Dependencies: {}
# [doc = " An extension trait for `Guard`."] pub trait GuardExt : Guard + Sized { # [doc = " Perform `and` operator on two rules"] fn and < R : Guard > (self , other : R) -> And < Self , R > { And (self , other) } # [doc = " Perform `or` operator on two rules"] fn or < R : Guard > (self , other : R) -> Or < Self , R > { Or (self , other) } }
};
}
