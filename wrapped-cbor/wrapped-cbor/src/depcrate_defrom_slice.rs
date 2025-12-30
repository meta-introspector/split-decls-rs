// Generated macro for from_slice (function)
macro_rules! Depcrate_defrom_slice {
() => {
// Module: crate::de
// Provides: {"from_slice"}
// Dependencies: {}
# [doc = " Decodes a value from CBOR data in a slice."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Deserialize a `String`"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_cbor::de;"] # [doc = " let v: Vec<u8> = vec![0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72];"] # [doc = " let value: String = de::from_slice(&v[..]).unwrap();"] # [doc = " assert_eq!(value, \"foobar\");"] # [doc = " ```"] # [doc = ""] # [doc = " Deserialize a borrowed string with zero copies."] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_cbor::de;"] # [doc = " let v: Vec<u8> = vec![0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72];"] # [doc = " let value: &str = de::from_slice(&v[..]).unwrap();"] # [doc = " assert_eq!(value, \"foobar\");"] # [doc = " ```"] # [cfg (any (feature = "std" , feature = "alloc"))] pub fn from_slice < 'a , T > (slice : & 'a [u8]) -> Result < T > where T : de :: Deserialize < 'a > , { let mut deserializer = Deserializer :: from_slice (slice) ; let value = de :: Deserialize :: deserialize (& mut deserializer) ? ; deserializer . end () ? ; Ok (value) }
};
}
