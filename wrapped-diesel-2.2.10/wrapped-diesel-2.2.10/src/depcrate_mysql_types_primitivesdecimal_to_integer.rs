// Generated macro for decimal_to_integer (function)
macro_rules! Depcrate_mysql_types_primitivesdecimal_to_integer {
() => {
// Module: crate::mysql::types::primitives
// Provides: {"decimal_to_integer"}
// Dependencies: {}
fn decimal_to_integer < T > (bytes : & [u8]) -> deserialize :: Result < T > where T : FromStr , T :: Err : Error + Send + Sync + 'static , { let string = str :: from_utf8 (bytes) ? ; let mut split = string . split ('.') ; let integer_portion = split . next () . unwrap_or_default () ; let _decimal_portion = split . next () . unwrap_or_default () ; if split . next () . is_some () { Err (format ! ("Invalid decimal format: {string:?}") . into ()) } else { Ok (integer_portion . parse () ?) } }
};
}
