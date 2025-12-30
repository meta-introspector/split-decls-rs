// Generated macro for decode_name (function)
macro_rules! Depcrate_de_keydecode_name {
() => {
// Module: crate::de::key
// Provides: {"decode_name"}
// Dependencies: {}
# [doc = " Decodes raw bytes using the deserializer encoding."] # [doc = " The method will borrow if encoding is UTF-8 compatible and `name` contains"] # [doc = " only UTF-8 compatible characters (usually only ASCII characters)."] # [inline] fn decode_name < 'n > (name : QName < 'n > , decoder : Decoder) -> Result < Cow < 'n , str > , DeError > { let local = name . local_name () ; Ok (decoder . decode (local . into_inner ()) ?) }
};
}
