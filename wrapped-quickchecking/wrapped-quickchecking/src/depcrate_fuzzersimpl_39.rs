// Generated macro for impl_39 (impl)
macro_rules! Depcrate_fuzzersimpl_39 {
() => {
// Module: crate::fuzzers
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " A qucickcheck trait for describing how `ArrayDimensionC` types can be"] # [doc = " randomly generated and shrunk."] impl Arbitrary for ArrayDimensionC { fn arbitrary (g : & mut Gen) -> ArrayDimensionC { let dimensions = gen_range (g , 0 , 5) ; let mut def = String :: new () ; let lower_bound = u64 :: from (cfg ! (feature = "zero-sized-arrays")) ; for _ in 1 .. dimensions { let _ = write ! (def , "[{}]" , gen_range (g , lower_bound , 16)) ; } ArrayDimensionC { def } } }
};
}
