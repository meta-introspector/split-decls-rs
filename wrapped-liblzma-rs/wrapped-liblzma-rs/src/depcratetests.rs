// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use quickcheck :: quickcheck ; # [cfg (all (target_family = "wasm" , target_os = "unknown"))] use wasm_bindgen_test :: wasm_bindgen_test as test ; # [test] fn all () { quickcheck (test as fn (_) -> _) ; fn test (v : Vec < u8 >) -> bool { let e = encode_all (& v [..] , 6) . unwrap () ; let d = decode_all (& e [..]) . unwrap () ; v == d } } # [test] fn copy () { quickcheck (test as fn (_) -> _) ; fn test (v : Vec < u8 >) -> bool { let mut e = Vec :: new () ; copy_encode (& v [..] , & mut e , 6) . unwrap () ; let mut d = Vec :: new () ; copy_decode (& e [..] , & mut d) . unwrap () ; v == d } } # [test] # [cfg (feature = "bindgen")] fn size () { quickcheck (test as fn (_) -> _) ; fn test (v : Vec < u8 >) -> bool { let mut e = Vec :: new () ; copy_encode (& v [..] , & mut e , 6) . unwrap () ; let s = super :: uncompressed_size (std :: io :: Cursor :: new (e)) . unwrap () ; (s as usize) == v . len () } } }
};
}
