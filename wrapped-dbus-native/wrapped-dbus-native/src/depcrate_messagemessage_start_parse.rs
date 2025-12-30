// Generated macro for message_start_parse (function)
macro_rules! Depcrate_messagemessage_start_parse {
() => {
// Module: crate::message
// Provides: {"message_start_parse"}
// Dependencies: {}
fn message_start_parse (buf : & [u8]) -> Result < MsgStart , DemarshalError > { if buf . len () < FIXED_HEADER_SIZE { Err (DemarshalError :: NotEnoughData) ? } ; if buf [3] != 1 { Err (DemarshalError :: InvalidProtocol) ? } ; let body_len = buf [4 .. 8] . try_into () . unwrap () ; let serial = buf [8 .. 12] . try_into () . unwrap () ; let arr_len = buf [12 .. 16] . try_into () . unwrap () ; let (is_big_endian , body_len , serial , arr_len) = match buf [0] { b'l' => (false , u32 :: from_le_bytes (body_len) , u32 :: from_le_bytes (serial) , u32 :: from_le_bytes (arr_len)) , b'B' => (true , u32 :: from_be_bytes (body_len) , u32 :: from_be_bytes (serial) , u32 :: from_be_bytes (arr_len)) , _ => Err (DemarshalError :: InvalidProtocol) ? } ; let body_len = body_len as usize ; let body_start = types :: align_up (arr_len as usize , 8) + FIXED_HEADER_SIZE ; let total_size = body_start + body_len ; if body_len >= 134217728 || arr_len >= 67108864 || total_size >= 134217728 { Err (DemarshalError :: NumberTooBig) ? } let serial = NonZeroU32 :: new (serial) . ok_or (DemarshalError :: NotEnoughData) ? ; Ok (MsgStart { total_size , serial , body_start , is_big_endian }) }
};
}
