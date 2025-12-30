// Generated macro for Value (enum)
macro_rules! Depcrate_stream_binary_writerValue {
() => {
// Module: crate::stream::binary_writer
// Provides: {"Value"}
// Dependencies: {}
# [derive (Eq , Hash , PartialEq)] enum Value < 'a > { Boolean (bool) , Data (Cow < 'a , [u8] >) , Date (Date) , Integer (Integer) , # [doc = " Floats are deduplicated based on their bitwise value."] Real (u64) , String (Cow < 'a , str >) , Uid (Uid) , }
};
}
