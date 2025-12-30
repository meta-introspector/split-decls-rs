// Generated macro for data_and_chunks (function)
macro_rules! Depcratedata_and_chunks {
() => {
// Module: crate
// Provides: {"data_and_chunks"}
// Dependencies: {}
fn data_and_chunks () -> impl Strategy < Value = (Vec < u8 > , Vec < Vec < u8 > >) > { prop :: collection :: vec (prop :: collection :: vec (num :: u8 :: ANY , 0 .. 100) , 0 .. 100) . prop_map (| vs | { let data = vs . iter () . flatten () . copied () . collect () ; (data , vs) }) }
};
}
