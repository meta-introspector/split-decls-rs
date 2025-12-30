// Generated macro for impl_8 (impl)
macro_rules! Depcrate_assignmentimpl_8 {
() => {
// Module: crate::assignment
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a > From < AssignmentRef < 'a > > for Assignment { fn from (a : AssignmentRef < 'a >) -> Self { Assignment { name : a . name . to_owned () , state : a . state . to_owned () , } } }
};
}
