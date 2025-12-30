// Generated macro for StreamDeserializer (struct)
macro_rules! Depcrate_deStreamDeserializer {
() => {
// Module: crate::de
// Provides: {"StreamDeserializer"}
// Dependencies: {}
# [doc = " Iterator that deserializes a stream into multiple CBOR values."] # [doc = ""] # [doc = " A stream deserializer can be created from any CBOR deserializer using the"] # [doc = " `Deserializer::into_iter` method."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate serde_cbor;"] # [doc = " use serde_cbor::de::Deserializer;"] # [doc = " use serde_cbor::value::Value;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = " let data: Vec<u8> = vec!["] # [doc = "     0x01, 0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72,"] # [doc = " ];"] # [doc = " let mut it = Deserializer::from_slice(&data[..]).into_iter::<Value>();"] # [doc = " assert_eq!("] # [doc = "     Value::Integer(1),"] # [doc = "     it.next().unwrap().unwrap()"] # [doc = " );"] # [doc = " assert_eq!("] # [doc = "     Value::Text(\"foobar\".to_string()),"] # [doc = "     it.next().unwrap().unwrap()"] # [doc = " );"] # [doc = " # }"] # [doc = " ```"] # [derive (Debug)] pub struct StreamDeserializer < 'de , R , T > { de : Deserializer < R > , output : PhantomData < T > , lifetime : PhantomData < & 'de () > , }
};
}
