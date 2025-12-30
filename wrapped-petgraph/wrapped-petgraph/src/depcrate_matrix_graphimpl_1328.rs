// Generated macro for impl_1328 (impl)
macro_rules! Depcrate_matrix_graphimpl_1328 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1328"}
// Dependencies: {}
impl < T , S > Index < usize > for IdStorage < T , S > { type Output = T ; fn index (& self , index : usize) -> & T { self . elements [index] . as_ref () . unwrap () } }
};
}
