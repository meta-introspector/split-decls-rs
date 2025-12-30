// Generated macro for deserializer_from_reader_with_buffer (function)
macro_rules! Depcrate_dedeserializer_from_reader_with_buffer {
() => {
// Module: crate::de
// Provides: {"deserializer_from_reader_with_buffer"}
// Dependencies: {}
# [doc = " Returns a deserializer with a specified scratch buffer"] # [inline] pub fn deserializer_from_reader_with_buffer < R : Read > (reader : R , scratch_buffer : & mut [u8] ,) -> Deserializer < '_ , R > where R :: Error : core :: fmt :: Debug , { Deserializer { decoder : reader . into () , scratch : scratch_buffer , recurse : 256 , } }
};
}
