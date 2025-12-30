// Generated macro for iteration (function)
macro_rules! Depcrate_laplaceiteration {
() => {
// Module: crate::laplace
// Provides: {"iteration"}
// Dependencies: {}
fn iteration (cur : & [f64] , next : & mut [f64] , size_x : usize , size_y : usize) { next . par_chunks_mut (size_y) . enumerate () . for_each (| (chunk_index , slice) | { if chunk_index > 0 && chunk_index < size_y - 1 { let offset_base = chunk_index * size_x ; for x in 1 .. size_x - 1 { slice [x] = (cur [offset_base + x - 1] + cur [offset_base + x + 1] + cur [offset_base + size_x + x] + cur [offset_base - size_x + x]) * 0.25 ; } } }) ; }
};
}
