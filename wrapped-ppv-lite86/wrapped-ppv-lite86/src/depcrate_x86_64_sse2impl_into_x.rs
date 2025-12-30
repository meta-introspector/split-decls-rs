// Generated macro for impl_into_x (macro)
macro_rules! Depcrate_x86_64_sse2impl_into_x {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_into_x"}
// Dependencies: {}
macro_rules ! impl_into_x { ($ from : ident , $ to : ident) => { impl < S3 : Copy , S4 : Copy , NI : Copy , Gf , Gt > From < x2 <$ from < S3 , S4 , NI >, Gf >> for x2 <$ to < S3 , S4 , NI >, Gt > { # [inline (always)] fn from (x : x2 <$ from < S3 , S4 , NI >, Gf >) -> Self { x2 :: new ([$ to :: from (x . 0 [0]) , $ to :: from (x . 0 [1])]) } } impl < S3 : Copy , S4 : Copy , NI : Copy > From < x4 <$ from < S3 , S4 , NI >>> for x4 <$ to < S3 , S4 , NI >> { # [inline (always)] fn from (x : x4 <$ from < S3 , S4 , NI >>) -> Self { x4 :: new ([$ to :: from (x . 0 [0]) , $ to :: from (x . 0 [1]) , $ to :: from (x . 0 [2]) , $ to :: from (x . 0 [3]) ,]) } } } ; }
};
}
