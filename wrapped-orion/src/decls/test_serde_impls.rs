macro_rules! test_serde_impls {
    () => {
        # [doc = ""] # [doc = " Test implementation macros"] # [cfg (test)] # [cfg (feature = "serde")] macro_rules ! test_serde_impls (($ name : ident , $ gen_length : expr) => (# [test] fn test_serde_serialized_equivalence_to_bytes_fn () { let bytes = & [38u8 ; $ gen_length] [..] ; let orion_type = $ name :: from_slice (bytes) . unwrap () ; let serialized_from_bytes = serde_json :: to_value (bytes) . unwrap () ; let serialized_from_orion_type = serde_json :: to_value (& orion_type) . unwrap () ; assert_eq ! (serialized_from_bytes , serialized_from_orion_type) ; } # [test] fn test_serde_deserialized_equivalence_to_bytes_fn () { let bytes = & [38u8 ; $ gen_length] [..] ; let serialized_from_bytes = serde_json :: to_value (bytes) . unwrap () ; let orion_type : $ name = serde_json :: from_value (serialized_from_bytes) . unwrap () ; assert_eq ! (orion_type , bytes) ; })) ;
    };
}

test_serde_impls!()