// Generated macro for unsafe_impl_endian_pod (macro)
macro_rules! Depcrate_endianunsafe_impl_endian_pod {
() => {
// Module: crate::endian
// Provides: {"unsafe_impl_endian_pod"}
// Dependencies: {}
macro_rules ! unsafe_impl_endian_pod { ($ ($ struct_name : ident) ,+ $ (,) ?) => { $ (unsafe impl < E : Endian > Pod for $ struct_name < E > { }) + } }
};
}
