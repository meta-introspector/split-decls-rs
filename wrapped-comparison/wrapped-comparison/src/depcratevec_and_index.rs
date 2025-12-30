// Generated macro for vec_and_index (function)
macro_rules! Depcratevec_and_index {
() => {
// Module: crate
// Provides: {"vec_and_index"}
// Dependencies: {}
fn vec_and_index () -> impl Strategy < Value = (Vec < u8 > , usize) > { prop :: collection :: vec (num :: u8 :: ANY , 0 ..= 32 * 1024) . prop_flat_map (| vec | { let len = vec . len () ; (Just (vec) , 0 ..= len) }) }
};
}
