// Generated macro for from_integers (macro)
macro_rules! Depcrate_header_valuefrom_integers {
() => {
// Module: crate::header::value
// Provides: {"from_integers"}
// Dependencies: {}
macro_rules ! from_integers { ($ ($ name : ident : $ t : ident => $ max_len : expr) ,*) => { $ (impl From <$ t > for HeaderValue { fn from (num : $ t) -> HeaderValue { let mut buf = BytesMut :: with_capacity ($ max_len) ; let _ = buf . write_str (:: itoa :: Buffer :: new () . format (num)) ; HeaderValue { inner : buf . freeze () , is_sensitive : false , } } } # [test] fn $ name () { let n : $ t = 55 ; let val = HeaderValue :: from (n) ; assert_eq ! (val , & n . to_string ()) ; let n = :: std ::$ t :: MAX ; let val = HeaderValue :: from (n) ; assert_eq ! (val , & n . to_string ()) ; }) * } ; }
};
}
