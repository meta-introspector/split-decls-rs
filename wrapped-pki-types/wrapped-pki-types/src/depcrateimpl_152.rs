// Generated macro for impl_152 (impl)
macro_rules! Depcrateimpl_152 {
() => {
// Module: crate
// Provides: {"impl_152"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a [u8] > for PrivateKeyDer < 'a > { type Error = & 'static str ; fn try_from (key : & 'a [u8]) -> Result < Self , Self :: Error > { const SHORT_FORM_LEN_MAX : u8 = 128 ; const TAG_SEQUENCE : u8 = 0x30 ; const TAG_INTEGER : u8 = 0x02 ; if key . first () != Some (& TAG_SEQUENCE) || key . len () < 2 { return Err (INVALID_KEY_DER_ERR) ; } let skip_len = match key [1] >= SHORT_FORM_LEN_MAX { false => 2 , true => 2 + (key [1] - SHORT_FORM_LEN_MAX) as usize , } ; let key_bytes = key . get (skip_len ..) . ok_or (INVALID_KEY_DER_ERR) ? ; if matches ! (key_bytes , [TAG_INTEGER , 0x01 , _ , TAG_SEQUENCE , ..]) { return Ok (Self :: Pkcs8 (key . into ())) ; } if key_bytes . starts_with (& [TAG_INTEGER , 0x01 , 0x00]) { return Ok (Self :: Pkcs1 (key . into ())) ; } if key_bytes . starts_with (& [TAG_INTEGER , 0x01 , 0x01]) { return Ok (Self :: Sec1 (key . into ())) ; } Err (INVALID_KEY_DER_ERR) } }
};
}
