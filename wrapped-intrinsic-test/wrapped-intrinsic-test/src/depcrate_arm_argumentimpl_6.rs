// Generated macro for impl_6 (impl)
macro_rules! Depcrate_arm_argumentimpl_6 {
() => {
// Module: crate::arm::argument
// Provides: {"impl_6"}
// Dependencies: {}
impl Argument < ArmIntrinsicType > { pub fn type_and_name_from_c (arg : & str) -> (& str , & str) { let split_index = arg . rfind ([' ' , '*']) . expect ("Couldn't split type and argname") ; (arg [.. split_index + 1] . trim_end () , & arg [split_index + 1 ..]) } }
};
}
