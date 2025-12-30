// Generated macro for unsafe_impl_pod (macro)
macro_rules! Depcrate_podunsafe_impl_pod {
() => {
// Module: crate::pod
// Provides: {"unsafe_impl_pod"}
// Dependencies: {}
macro_rules ! unsafe_impl_pod { ($ ($ struct_name : ident) ,+ $ (,) ?) => { $ (unsafe impl Pod for $ struct_name { }) + } }
};
}
