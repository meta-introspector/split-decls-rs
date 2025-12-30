// Generated macro for impl_33 (impl)
macro_rules! Depcrate_fuzzersimpl_33 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_33"}
// Dependencies: {}
# [doc = " A quickcheck trait for describing how `BaseTypeC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for BaseTypeC { fn arbitrary (g : & mut Gen) -> BaseTypeC { let base_type = vec ! ["char" , "signed char" , "unsigned char" , "short" , "short int" , "signed short" , "signed short int" , "unsigned short" , "unsigned short int" , "int" , "signed" , "signed int" , "unsigned" , "unsigned int" , "long" , "long int" , "signed long" , "signed long int" , "unsigned long" , "unsigned long int" , "long long" , "long long int" , "signed long long" , "signed long long int" , "unsigned long long" , "unsigned long long int" , "float" , "double" , # [cfg (feature = "long-doubles")] "long double" , "void*" ,] ; BaseTypeC { def : String :: from (* g . choose (& base_type) . unwrap ()) , } } }
};
}
