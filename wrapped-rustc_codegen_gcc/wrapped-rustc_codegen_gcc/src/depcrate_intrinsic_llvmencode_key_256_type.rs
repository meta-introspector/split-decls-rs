// Generated macro for encode_key_256_type (function)
macro_rules! Depcrate_intrinsic_llvmencode_key_256_type {
() => {
// Module: crate::intrinsic::llvm
// Provides: {"encode_key_256_type"}
// Dependencies: {}
fn encode_key_256_type < 'a , 'gcc , 'tcx > (builder : & Builder < 'a , 'gcc , 'tcx > ,) -> (Type < 'gcc > , Field < 'gcc > , Field < 'gcc >) { let m128i = builder . context . new_vector_type (builder . i64_type , 2) ; let field1 = builder . context . new_field (None , builder . u32_type , "field1") ; let field2 = builder . context . new_field (None , m128i , "field2") ; let field3 = builder . context . new_field (None , m128i , "field3") ; let field4 = builder . context . new_field (None , m128i , "field4") ; let field5 = builder . context . new_field (None , m128i , "field5") ; let field6 = builder . context . new_field (None , m128i , "field6") ; let field7 = builder . context . new_field (None , m128i , "field7") ; let field8 = builder . context . new_field (None , m128i , "field8") ; let encode_type = builder . context . new_struct_type (None , "EncodeKey256Output" , & [field1 , field2 , field3 , field4 , field5 , field6 , field7 , field8] ,) ; # [cfg (feature = "master")] encode_type . as_type () . set_packed () ; (encode_type . as_type () , field1 , field2) }
};
}
