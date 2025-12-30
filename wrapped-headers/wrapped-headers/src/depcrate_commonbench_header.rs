// Generated macro for bench_header (macro)
macro_rules! Depcrate_commonbench_header {
() => {
// Module: crate::common
// Provides: {"bench_header"}
// Dependencies: {}
# [cfg (test)] macro_rules ! bench_header { ($ mod : ident , $ ty : ident , $ value : expr) => { # [cfg (feature = "nightly")] mod $ mod { use super ::$ ty ; use crate :: HeaderMapExt ; # [bench] fn bench_decode (b : & mut :: test :: Bencher) { let mut map = :: http :: HeaderMap :: new () ; map . append (<$ ty as crate :: Header >:: name () , $ value . parse () . expect ("HeaderValue::from_str($value)") ,) ; b . bytes = $ value . len () as u64 ; b . iter (|| { map . typed_get ::<$ ty > () . unwrap () ; }) ; } # [bench] fn bench_encode (b : & mut :: test :: Bencher) { let mut map = :: http :: HeaderMap :: new () ; map . append (<$ ty as crate :: Header >:: name () , $ value . parse () . expect ("HeaderValue::from_str($value)") ,) ; let typed = map . typed_get ::<$ ty > () . unwrap () ; b . bytes = $ value . len () as u64 ; b . iter (|| { map . typed_insert (typed . clone ()) ; map . clear () ; }) ; } } } ; }
};
}
