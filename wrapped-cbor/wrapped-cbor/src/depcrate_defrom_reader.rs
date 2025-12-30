// Generated macro for from_reader (function)
macro_rules! Depcrate_defrom_reader {
() => {
// Module: crate::de
// Provides: {"from_reader"}
// Dependencies: {}
# [doc = " Decodes a value from CBOR data in a reader."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Deserialize a `String`"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_cbor::de;"] # [doc = " let v: Vec<u8> = vec![0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72];"] # [doc = " let value: String = de::from_reader(&v[..]).unwrap();"] # [doc = " assert_eq!(value, \"foobar\");"] # [doc = " ```"] # [doc = ""] # [doc = " Note that `from_reader` cannot borrow data:"] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " # use serde_cbor::de;"] # [doc = " let v: Vec<u8> = vec![0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72];"] # [doc = " let value: &str = de::from_reader(&v[..]).unwrap();"] # [doc = " assert_eq!(value, \"foobar\");"] # [doc = " ```"] # [cfg (feature = "std")] pub fn from_reader < T , R > (reader : R) -> Result < T > where T : de :: DeserializeOwned , R : io :: Read , { let mut deserializer = Deserializer :: from_reader (reader) ; let value = de :: Deserialize :: deserialize (& mut deserializer) ? ; deserializer . end () ? ; Ok (value) }
};
}
