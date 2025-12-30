// Generated macro for test_to_string_pretty_complex (function)
macro_rules! Depcrate_content_jsontest_to_string_pretty_complex {
() => {
// Module: crate::content::json
// Provides: {"test_to_string_pretty_complex"}
// Dependencies: {}
# [test] fn test_to_string_pretty_complex () { let content = Content :: Map (vec ! [(Content :: from ("is_alive") , Content :: NewtypeStruct ("Some" , Content :: from (true) . into ()) ,) , (Content :: from ("newtype_variant") , Content :: NewtypeVariant ("Foo" , 0 , "variant_a" , Box :: new (Content :: Struct ("VariantA" , vec ! [("field_a" , Content :: String ("value_a" . into ())) , ("field_b" , 42u32 . into ()) ,] ,)) ,) ,) , (Content :: from ("struct_variant") , Content :: StructVariant ("Foo" , 0 , "variant_b" , vec ! [("field_a" , Content :: String ("value_a" . into ())) , ("field_b" , 42u32 . into ()) ,] ,) ,) , (Content :: from ("tuple_variant") , Content :: TupleVariant ("Foo" , 0 , "variant_c" , vec ! [(Content :: String ("value_a" . into ())) , (42u32 . into ())] ,) ,) , (Content :: from ("empty_array") , Content :: Seq (vec ! [])) , (Content :: from ("empty_object") , Content :: Map (vec ! [])) , (Content :: from ("array") , Content :: Seq (vec ! [true . into ()])) , (Content :: from ("object") , Content :: Map (vec ! [("foo" . into () , true . into ())]) ,) , (Content :: from ("array_of_objects") , Content :: Seq (vec ! [Content :: Struct ("MyType" , vec ! [("foo" , Content :: from ("bar" . to_string ())) , ("bar" , Content :: from ("xxx" . to_string ())) ,] ,)]) ,) , (Content :: from ("unit_variant") , Content :: UnitVariant ("Stuff" , 0 , "value") ,) , (Content :: from ("u8") , Content :: U8 (8)) , (Content :: from ("u16") , Content :: U16 (16)) , (Content :: from ("u32") , Content :: U32 (32)) , (Content :: from ("u64") , Content :: U64 (64)) , (Content :: from ("u128") , Content :: U128 (128)) , (Content :: from ("i8") , Content :: I8 (8)) , (Content :: from ("i16") , Content :: I16 (16)) , (Content :: from ("i32") , Content :: I32 (32)) , (Content :: from ("i64") , Content :: I64 (64)) , (Content :: from ("i128") , Content :: I128 (128)) , (Content :: from ("f32") , Content :: F32 (32.0)) , (Content :: from ("f64") , Content :: F64 (64.0)) , (Content :: from ("char") , Content :: Char ('A')) , (Content :: from ("bytes") , Content :: Bytes (b"hehe" . to_vec ())) , (Content :: from ("null") , Content :: None) , (Content :: from ("unit") , Content :: Unit) , (Content :: from ("crazy_string") , Content :: String ((0u8 ..= 126) . map (| x | x as char) . collect ()) ,) ,]) ; let json = to_string_pretty (& content) ; crate :: assert_snapshot ! (& json , @ r##"
    {
      "is_alive": true,
      "newtype_variant": {
        "variant_a": {
          "field_a": "value_a",
          "field_b": 42
        }
      },
      "struct_variant": {
        "variant_b": {
          "field_a": "value_a",
          "field_b": 42
        }
      },
      "tuple_variant": {
        "variant_c": [
          "value_a",
          42
        ]
      },
      "empty_array": [],
      "empty_object": {},
      "array": [
        true
      ],
      "object": {
        "foo": true
      },
      "array_of_objects": [
        {
          "foo": "bar",
          "bar": "xxx"
        }
      ],
      "unit_variant": "value",
      "u8": 8,
      "u16": 16,
      "u32": 32,
      "u64": 64,
      "u128": 128,
      "i8": 8,
      "i16": 16,
      "i32": 32,
      "i64": 64,
      "i128": 128,
      "f32": 32.0,
      "f64": 64.0,
      "char": "A",
      "bytes": [
        104,
        101,
        104,
        101
      ],
      "null": null,
      "unit": null,
      "crazy_string": "\u0000\u0001\u0002\u0003\u0004\u0005\u0006\u0007\b\t\n\u000b\f\r\u000e\u000f\u0010\u0011\u0012\u0013\u0014\u0015\u0016\u0017\u0018\u0019\u001a\u001b\u001c\u001d\u001e\u001f !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
    }
    "##) ; }
};
}
