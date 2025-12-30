// Generated macro for test_complex_using_primitives (function)
macro_rules! Depcrate_teststest_complex_using_primitives {
() => {
// Module: crate::tests
// Provides: {"test_complex_using_primitives"}
// Dependencies: {}
# [test] # [cfg (feature = "num-complex")] fn test_complex_using_primitives () { fn expand_equiv_class_ratio (cls : & [N]) -> Vec < N > { let mut ret = Vec :: new () ; for e in cls { match e { N :: u8 (v) => ret . push (N :: u8 (* v)) , N :: u16 (v) => ret . push (N :: u16 (* v)) , N :: u32 (v) => ret . push (N :: u32 (* v)) , N :: u64 (v) => ret . push (N :: u64 (* v)) , N :: u128 (v) => ret . push (N :: u128 (* v)) , N :: i8 (v) => ret . push (N :: i8 (* v)) , N :: i16 (v) => ret . push (N :: i16 (* v)) , N :: i32 (v) => ret . push (N :: i32 (* v)) , N :: i64 (v) => ret . push (N :: i64 (* v)) , N :: i128 (v) => ret . push (N :: i128 (* v)) , N :: f64 (v) => ret . extend_from_slice (& [N :: f64 (* v) , N :: c64 (Complex :: from (* v))]) , N :: f32 (v) => ret . extend_from_slice (& [N :: f32 (* v) , N :: c32 (Complex :: from (* v)) , N :: f64 (* v as f64) , N :: c64 (Complex :: from (* v as f64))]) , _ => { } } } ret } let numbers : Vec < _ > = NUMBERS . iter () . map (| cls | expand_equiv_class_ratio (cls)) . collect () ; for icls in 0 .. numbers . len () { for jcls in 0 .. numbers . len () { let expected = icls . cmp (& jcls) ; for i in & numbers [icls] { for j in & numbers [jcls] { assert_cmp (i , j , expected) ; } } } } for & equiv in NUMBERS { let equiv = expand_equiv_class_ratio (equiv) ; let hashes : Vec < u64 > = equiv . iter () . map (hash) . collect () ; for i in 1 .. equiv . len () { assert_eq ! (hashes [0] , hashes [i] , "Hash mismatch between {:?} and {:?}" , equiv [0] , equiv [i]) ; } } }
};
}
