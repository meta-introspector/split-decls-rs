// Generated macro for impl_9 (impl)
macro_rules! Depcrate_assignmentimpl_9 {
() => {
// Module: crate::assignment
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'a > Assignment { # [doc = " Provide a ref type to this owned instance."] pub fn as_ref (& 'a self) -> AssignmentRef < 'a > { AssignmentRef :: new (self . name . as_ref () , self . state . as_ref ()) } }
};
}
