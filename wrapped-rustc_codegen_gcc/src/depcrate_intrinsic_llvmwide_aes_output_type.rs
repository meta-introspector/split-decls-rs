// Generated macro for wide_aes_output_type (function)
macro_rules! Depcrate_intrinsic_llvmwide_aes_output_type {
() => {
// Module: crate::intrinsic::llvm
// Provides: {"wide_aes_output_type"}
// Dependencies: {}
fn wide_aes_output_type < 'a , 'gcc , 'tcx > (builder : & Builder < 'a , 'gcc , 'tcx > ,) -> (Type < 'gcc > , Field < 'gcc > , Field < 'gcc >) { let m128i = builder . context . new_vector_type (builder . i64_type , 2) ; let field1 = builder . context . new_field (None , builder . u8_type , "field1") ; let field2 = builder . context . new_field (None , m128i , "field2") ; let field3 = builder . context . new_field (None , m128i , "field3") ; let field4 = builder . context . new_field (None , m128i , "field4") ; let field5 = builder . context . new_field (None , m128i , "field5") ; let field6 = builder . context . new_field (None , m128i , "field6") ; let field7 = builder . context . new_field (None , m128i , "field7") ; let field8 = builder . context . new_field (None , m128i , "field8") ; let field9 = builder . context . new_field (None , m128i , "field9") ; let aes_output_type = builder . context . new_struct_type (None , "WideAesOutput" , & [field1 , field2 , field3 , field4 , field5 , field6 , field7 , field8 , field9] ,) ; # [cfg (feature = "master")] aes_output_type . as_type () . set_packed () ; (aes_output_type . as_type () , field1 , field2) }
};
}
