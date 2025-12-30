// Generated macro for impl_7 (impl)
macro_rules! Depcrate_assignmentimpl_7 {
() => {
// Module: crate::assignment
// Provides: {"impl_7"}
// Dependencies: {}
impl < 'a > AssignmentRef < 'a > { pub (crate) fn new (name : NameRef < 'a > , state : StateRef < 'a >) -> AssignmentRef < 'a > { AssignmentRef { name , state } } # [doc = " Turn this reference into its owned counterpart."] pub fn to_owned (self) -> Assignment { self . into () } }
};
}
