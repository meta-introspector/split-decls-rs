// Generated macro for decode (function)
macro_rules! Depcratedecode {
() => {
// Module: crate
// Provides: {"decode"}
// Dependencies: {}
# [doc = " Determine the encoding by looking for a Byte Order Mark (BOM)"] # [doc = " and decoded a single string in memory."] # [doc = " Return the result and the used encoding."] pub fn decode (input : & [u8] , trap : DecoderTrap , fallback_encoding : EncodingRef) -> (Result < String , Cow < 'static , str > > , EncodingRef) { use all :: { UTF_8 , UTF_16LE , UTF_16BE } ; if input . starts_with (& [0xEF , 0xBB , 0xBF]) { (UTF_8 . decode (& input [3 ..] , trap) , UTF_8 as EncodingRef) } else if input . starts_with (& [0xFE , 0xFF]) { (UTF_16BE . decode (& input [2 ..] , trap) , UTF_16BE as EncodingRef) } else if input . starts_with (& [0xFF , 0xFE]) { (UTF_16LE . decode (& input [2 ..] , trap) , UTF_16LE as EncodingRef) } else { (fallback_encoding . decode (input , trap) , fallback_encoding) } }
};
}
