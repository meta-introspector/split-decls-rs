// Generated macro for aes_output_type (function)
macro_rules! Depcrate_intrinsic_llvmaes_output_type {
() => {
// Module: crate::intrinsic::llvm
// Provides: {"aes_output_type"}
// Dependencies: {}
fn aes_output_type < 'a , 'gcc , 'tcx > (builder : & Builder < 'a , 'gcc , 'tcx > ,) -> (Type < 'gcc > , Field < 'gcc > , Field < 'gcc >) { let m128i = builder . context . new_vector_type (builder . i64_type , 2) ; let field1 = builder . context . new_field (None , builder . u8_type , "field1") ; let field2 = builder . context . new_field (None , m128i , "field2") ; let aes_output_type = builder . context . new_struct_type (None , "AesOutput" , & [field1 , field2]) ; let typ = aes_output_type . as_type () ; # [cfg (feature = "master")] typ . set_packed () ; (typ , field1 , field2) }
};
}
