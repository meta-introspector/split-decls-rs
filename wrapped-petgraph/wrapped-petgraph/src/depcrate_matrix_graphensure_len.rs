// Generated macro for ensure_len (function)
macro_rules! Depcrate_matrix_graphensure_len {
() => {
// Module: crate::matrix_graph
// Provides: {"ensure_len"}
// Dependencies: {}
# [doc = " Grow a Vec by appending the type's default value until the `size` is reached."] fn ensure_len < T : Default > (v : & mut Vec < T > , size : usize) { v . resize_with (size , T :: default) ; }
};
}
