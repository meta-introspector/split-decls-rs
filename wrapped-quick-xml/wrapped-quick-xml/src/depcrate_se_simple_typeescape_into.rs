// Generated macro for escape_into (function)
macro_rules! Depcrate_se_simple_typeescape_into {
() => {
// Module: crate::se::simple_type
// Provides: {"escape_into"}
// Dependencies: {}
fn escape_into < W , F > (mut writer : W , value : & str , escape_chars : F) -> fmt :: Result where W : Write , F : Fn (u8) -> bool , { let bytes = value . as_bytes () ; let mut iter = bytes . iter () ; let mut pos = 0 ; while let Some (i) = iter . position (| & b | escape_chars (b)) { let new_pos = pos + i ; escape_char (& mut writer , value , pos , new_pos) ? ; pos = new_pos + 1 ; } if let Some (raw) = value . get (pos ..) { writer . write_str (raw) ? ; } Ok (()) }
};
}
