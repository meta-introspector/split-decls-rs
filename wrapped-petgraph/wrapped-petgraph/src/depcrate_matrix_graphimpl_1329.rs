// Generated macro for impl_1329 (impl)
macro_rules! Depcrate_matrix_graphimpl_1329 {
() => {
// Module: crate::matrix_graph
// Provides: {"impl_1329"}
// Dependencies: {}
impl < T , S > IndexMut < usize > for IdStorage < T , S > { fn index_mut (& mut self , index : usize) -> & mut T { self . elements [index] . as_mut () . unwrap () } }
};
}
