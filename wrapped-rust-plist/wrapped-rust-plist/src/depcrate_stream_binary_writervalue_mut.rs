// Generated macro for value_mut (function)
macro_rules! Depcrate_stream_binary_writervalue_mut {
() => {
// Module: crate::stream::binary_writer
// Provides: {"value_mut"}
// Dependencies: {}
fn value_mut < 'a > (values : & 'a mut IndexMap < Value < 'static > , ValueState > , value_index : usize ,) -> (& 'a Value < 'static > , & 'a mut ValueState) { values . get_index_mut (value_index) . expect ("internal consistency error") }
};
}
