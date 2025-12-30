// Generated macro for union (function)
macro_rules! Depcrate_confunion {
() => {
// Module: crate::conf
// Provides: {"union"}
// Dependencies: {}
fn union (x : & Range < usize > , y : & Range < usize >) -> Range < usize > { Range { start : cmp :: min (x . start , y . start) , end : cmp :: max (x . end , y . end) , } }
};
}
