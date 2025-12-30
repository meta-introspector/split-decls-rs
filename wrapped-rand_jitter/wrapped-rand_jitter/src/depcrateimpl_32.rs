// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < F > Clone for JitterRng < F > where F : Clone , { fn clone (& self) -> JitterRng < F > { JitterRng { data : self . data , rounds : self . rounds , timer : self . timer . clone () , mem_prev_index : self . mem_prev_index , data_half_used : false , } } }
};
}
